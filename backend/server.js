require('dotenv').config();
const express = require('express');
const http = require('http');
const WebSocket = require('ws');
const cors = require('cors');
const multer = require('multer');
const { PrismaClient } = require('@prisma/client');
const { analysisQueue } = require('./queue');

const app = express();
const server = http.createServer(app);
const wss = new WebSocket.Server({ server, path: '/stream' });
const PORT = process.env.PORT || 8000;
const prisma = new PrismaClient();

// WebSocket connection handling and broadcast helper
const clients = new Set();
wss.on('connection', (ws) => {
    console.log('[WebSocket] Client connected');
    clients.add(ws);
    ws.on('close', () => clients.delete(ws));
});

// Expose broadcast globally so queue.js can use it
global.broadcast = (topic, data) => {
    const payload = JSON.stringify({ topic, data });
    for (let client of clients) {
        if (client.readyState === WebSocket.OPEN) {
            client.send(payload);
        }
    }
};

const storage = multer.memoryStorage();
const upload = multer({ storage: storage });

app.use(cors());
app.use(express.json());

app.get('/', (req, res) => {
    res.json({ status: 'ok', service: 'simware-backend-node' });
});

// Real Analyze endpoint
app.post('/api/v1/analyze', upload.single('file'), async (req, res) => {
    try {
        const { file_path, file_hash } = req.body;
        
        if (!file_hash) {
            return res.status(400).json({ error: 'file_hash is required' });
        }

        // 1. Create a pending analysis record in the database
        const analysis = await prisma.analysis.create({
            data: {
                fileHash: file_hash,
                fileName: file_path || req.file?.originalname || 'unknown',
                status: 'pending'
            }
        });

        // 2. Queue the job for background processing
        await analysisQueue.add('AnalyzeBinary', {
            analysisId: analysis.id,
            fileHash: file_hash,
            filePath: file_path,
            // If there's an actual file buffer, we'd process it or save it here.
            // For now, we rely on the agent having the file locally or uploading it.
        });

        res.json({
            id: analysis.id,
            status: 'pending',
            message: `Analysis queued for hash ${file_hash}`
        });
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Failed to queue analysis' });
    }
});

// Real Status endpoint
app.get('/api/v1/analysis/:analysis_id', async (req, res) => {
    try {
        const { analysis_id } = req.params;
        
        const analysis = await prisma.analysis.findUnique({
            where: { id: analysis_id },
            include: { classifications: true, telemetryEvents: true }
        });

        if (!analysis) {
            return res.status(404).json({ error: 'Analysis not found' });
        }

        res.json(analysis);
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Failed to fetch analysis status' });
    }
});

// Dashboard Stats endpoint
app.get('/api/v1/stats', async (req, res) => {
    try {
        const totalAnalyzed = await prisma.analysis.count();
        const threatsBlocked = await prisma.analysis.count({ where: { finalDecision: 'BLOCK' } });
        
        const avgAnalysisTime = "1.2s"; // Mocked since we don't store completedAt yet

        const recent = await prisma.analysis.findMany({
            take: 5,
            orderBy: { createdAt: 'desc' }
        });

        res.json({
            totalAnalyzed,
            threatsBlocked,
            avgAnalysisTime,
            recentActivity: recent
        });
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Failed to fetch stats' });
    }
});

// Global Search endpoint
app.get('/api/v1/search', async (req, res) => {
    try {
        const { q } = req.query;
        if (!q) return res.json([]);
        
        const results = await prisma.analysis.findMany({
            where: {
                OR: [
                    { fileHash: { contains: q } },
                    { fileName: { contains: q } }
                ]
            },
            take: 20,
            orderBy: { createdAt: 'desc' }
        });
        res.json(results);
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Search failed' });
    }
});

// Export Report endpoint
app.get('/api/v1/reports/:id', async (req, res) => {
    try {
        const { id } = req.params;
        const analysis = await prisma.analysis.findUnique({
            where: { id },
            include: { classifications: true, telemetryEvents: true }
        });
        
        if (!analysis) return res.status(404).json({ error: 'Not found' });
        
        res.setHeader('Content-disposition', `attachment; filename=simware_report_${id}.json`);
        res.setHeader('Content-type', 'application/json');
        res.send(JSON.stringify(analysis, null, 2));
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Failed to generate report' });
    }
});

// Manual Scan Trigger (For Demo/Dashboard)
app.post('/api/v1/scan/manual', async (req, res) => {
    try {
        const { file_name } = req.body;
        const dummyHash = require('crypto').createHash('sha256').update((file_name || 'unknown') + Date.now()).digest('hex');
        
        const analysis = await prisma.analysis.create({
            data: {
                fileHash: dummyHash,
                fileName: file_name || 'Manual Upload',
                status: 'pending'
            }
        });

        await analysisQueue.add('AnalyzeBinary', {
            analysisId: analysis.id,
            fileHash: dummyHash,
            filePath: file_name || 'Manual Upload',
        });

        res.json({ id: analysis.id, status: 'pending', message: 'Manual scan queued' });
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Failed to trigger scan' });
    }
});

server.listen(PORT, () => {
    console.log(`Simware Backend API (and WebSocket Stream) running on http://localhost:${PORT}`);
});
