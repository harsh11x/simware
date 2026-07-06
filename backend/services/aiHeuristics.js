const axios = require('axios');

class AiHeuristicsService {
    constructor() {
        // In a production environment, this would come from process.env.GEMINI_API_KEY
        this.apiKey = process.env.GEMINI_API_KEY || 'MOCK_API_KEY';
        this.apiUrl = 'https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent';
    }

    /**
     * Analyzes dynamic telemetry logs to determine intent and risk score.
     * @param {Array<string>} telemetryLogs An array of log strings from QEMU
     * @returns {Object} { riskScore: number, explanation: string, decision: 'ALLOW' | 'BLOCK' }
     */
    async analyzeTelemetry(telemetryLogs) {
        const logString = telemetryLogs.join('\n');
        
        if (this.apiKey === 'MOCK_API_KEY') {
            console.log(`[AI Heuristics] Mocking Gemini response for ${telemetryLogs.length} logs`);
            // Simple keyword-based mock if we don't have an API key
            if (logString.toLowerCase().includes('lsass') || logString.toLowerCase().includes('registry')) {
                return {
                    riskScore: 0.92,
                    decision: 'BLOCK',
                    explanation: 'The process attempted to inject code into the Local Security Authority Subsystem Service (lsass.exe), a common credential dumping technique.'
                };
            }
            return {
                riskScore: 0.15,
                decision: 'ALLOW',
                explanation: 'The file executed normally without exhibiting any known malicious behavioral patterns.'
            };
        }

        const prompt = `
        You are a senior malware analyst. Analyze the following behavioral execution logs from a sandboxed environment.
        Determine if the behavior is malicious.
        Respond ONLY with a JSON object in this exact format, with no markdown formatting or backticks:
        { "riskScore": <float 0.0-1.0>, "decision": "ALLOW" or "BLOCK", "explanation": "<short string>" }

        Execution Logs:
        ${logString}
        `;

        try {
            const response = await axios.post(`${this.apiUrl}?key=${this.apiKey}`, {
                contents: [{ parts: [{ text: prompt }] }]
            });

            let textResponse = response.data.candidates[0].content.parts[0].text;
            // Clean up any markdown blocks if the model ignored the instruction
            textResponse = textResponse.replace(/```json/g, '').replace(/```/g, '').trim();
            
            return JSON.parse(textResponse);
        } catch (error) {
            console.error('[AI Heuristics] API Error:', error.message);
            // Safe default fallback
            return {
                riskScore: 0.5,
                decision: 'BLOCK',
                explanation: 'Analysis failed. Blocked out of an abundance of caution.'
            };
        }
    }
}

module.exports = new AiHeuristicsService();
