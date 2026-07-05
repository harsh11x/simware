const { Worker } = require('bullmq');
const Redis = require('ioredis');
const { PrismaClient } = require('@prisma/client');
require('dotenv').config();

const prisma = new PrismaClient();
const redisConnection = new Redis(process.env.REDIS_URL || 'redis://localhost:6379');

const worker = new Worker('AnalysisQueue', async (job) => {
    const { analysisId, filePath, fileHash } = job.data;
    
    console.log(`[Worker] Started processing analysis ${analysisId} for hash ${fileHash}`);
    
    // 1. Update status to 'processing'
    await prisma.analysis.update({
        where: { id: analysisId },
        data: { status: 'processing' }
    });

    // 2. Simulate Static Analysis & AI Risk Scoring
    console.log(`[Worker] Performing static analysis...`);
    await new Promise(resolve => setTimeout(resolve, 2000)); // Mock delay
    
    const mockAiScore = Math.random() * 100;
    const isMalicious = mockAiScore > 75;
    
    // 3. Simulate Sandbox execution if score is concerning, or just jump to verdict
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
}, { connection: redisConnection });

worker.on('completed', (job) => {
    console.log(`Job ${job.id} has completed!`);
});

worker.on('failed', (job, err) => {
    console.log(`Job ${job.id} has failed with ${err.message}`);
});

console.log("Analysis Worker started, listening to AnalysisQueue...");
