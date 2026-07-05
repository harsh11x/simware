const { PrismaClient } = require('@prisma/client');
const prisma = new PrismaClient();

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

        // 2. Real Orchestration would happen here (call static analysis tools)
        console.log(`[Worker] Orchestrating Static Analysis...`);
        await new Promise(resolve => setTimeout(resolve, 2000));
        
        const mockAiScore = Math.random() * 100;
        const isMalicious = mockAiScore > 75;
        
        console.log(`[Worker] Correlating telemetry... AI Score: ${mockAiScore.toFixed(2)}`);
        
        const decision = isMalicious ? 'BLOCK' : 'ALLOW';
        const status = 'completed';

        // 4. Update final results
        await prisma.analysis.update({
            where: { id: analysisId },
            data: { 
                status: status,
                aiRiskScore: mockAiScore,
                finalDecision: decision
            }
        });

        if (isMalicious) {
            await prisma.threatClassification.create({
                data: {
                    analysisId: analysisId,
                    category: mockAiScore > 90 ? 'Ransomware' : 'Trojan',
                    confidence: mockAiScore
                }
            });
        }
        
        console.log(`[Worker] Finished processing ${analysisId}. Decision: ${decision}`);
    }
}

const analysisQueue = new InMemoryQueue();

module.exports = {
    analysisQueue
};
