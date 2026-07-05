require('dotenv').config();
const express = require('express');
const cors = require('cors');
const multer = require('multer');
const { PrismaClient } = require('@prisma/client');
const { analysisQueue } = require('./queue');

const app = express();
const PORT = process.env.PORT || 8000;
const prisma = new PrismaClient();

// Configure Multer for in-memory file uploads (for MVP)
// In production, we'd save to disk or S3
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

app.listen(PORT, () => {
    console.log(`Simware Backend API running on http://localhost:${PORT}`);
});
