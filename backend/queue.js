const { PrismaClient } = require('@prisma/client');
const prisma = new PrismaClient();

const threatIntel = require('./services/threatIntel');
const aiHeuristics = require('./services/aiHeuristics');

// Simple in-memory queue since Docker/Redis is not available locally
class InMemoryQueue {
    constructor() {
        this.jobs = [];
        this.processing = false;
    }

    async add(name, data) {
        this.jobs.push({ name, data });
        console.log(`[Queue] Added job: ${name}`);
        this.processNext();
    }

    async processNext() {
        if (this.processing || this.jobs.length === 0) return;
        this.processing = true;

        const job = this.jobs.shift();
        try {
            await this.processJob(job);
        } catch (error) {
            console.error(`[Queue] Job failed: ${error.message}`);
        } finally {
            this.processing = false;
            this.processNext();
        }
    }

    async processJob(job) {
        const { analysisId, filePath, fileHash } = job.data;
        
        console.log(`[Worker] Started processing analysis ${analysisId} for hash ${fileHash}`);
        
        // 1. Update status to 'processing'
        await prisma.analysis.update({
            where: { id: analysisId },
            data: { status: 'processing' }
        });
        
        if (global.broadcast) global.broadcast('analysis_start', { analysisId });

        // STAGE 1: STATIC THREAT INTEL (VirusTotal)
        console.log(`[Worker] Stage 1: Querying Threat Intel for ${fileHash}...`);
        const vtReport = await threatIntel.queryFileHash(fileHash);
        
        if (vtReport.isMalicious) {
            console.log(`[Worker] VT returned MALICIOUS. Short-circuiting to BLOCK.`);
            await this.finalizeAnalysis(analysisId, 1.0, 'BLOCK', `Static Analysis Match: ${vtReport.classification} (${vtReport.enginesDetected} engines)`);
            return;
        }

        // STAGE 2: DYNAMIC SANDBOX EXECUTION (QEMU Simulation)
        console.log(`[Worker] Stage 2: Orchestrating Dynamic Sandbox...`);
        if (global.broadcast) global.broadcast('telemetry_log', { analysisId, log: '[QEMU Sandbox] Booting from hibernation snapshot...' });
        
        // Simulate Sandbox execution logs (In reality, QEMU/Agent streams this)
        const simLogs = [
            `[Sysmon] Process created: ${filePath || 'unknown.exe'}`,
            `[Sysmon] Network connection attempt to 185.12.X.X:443`,
            `[Sysmon] File write blocked: C:\\Windows\\System32\\malware.dll`
        ];
        
        for (let log of simLogs) {
            await new Promise(r => setTimeout(r, 600)); // Simulate time passing
            if (global.broadcast) global.broadcast('telemetry_log', { analysisId, log });
        }

        // STAGE 3: AI BEHAVIORAL HEURISTICS (Gemini)
        console.log(`[Worker] Stage 3: Requesting AI Behavioral Analysis...`);
        const aiVerdict = await aiHeuristics.analyzeTelemetry(simLogs);
        
        console.log(`[Worker] AI Verdict: ${aiVerdict.decision} (Risk: ${aiVerdict.riskScore})`);
        
        await this.finalizeAnalysis(analysisId, aiVerdict.riskScore, aiVerdict.decision, aiVerdict.explanation);
    }
    
    async finalizeAnalysis(analysisId, riskScore, decision, explanation) {
        await prisma.analysis.update({
            where: { id: analysisId },
            data: { 
                status: 'completed',
                aiRiskScore: riskScore,
                finalDecision: decision
            }
        });

        if (decision === 'BLOCK') {
            await prisma.threatClassification.create({
                data: {
                    analysisId: analysisId,
                    category: explanation,
                    confidence: riskScore * 100
                }
            });
        }
        
        if (global.broadcast) global.broadcast('analysis_complete', { analysisId, decision, riskScore, explanation });
        console.log(`[Worker] Finished processing ${analysisId}. Decision: ${decision}`);
    }
}

const analysisQueue = new InMemoryQueue();

module.exports = {
    analysisQueue
};
