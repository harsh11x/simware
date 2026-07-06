const axios = require('axios');

class ThreatIntelService {
    constructor() {
        // In a production environment, this would come from process.env.VIRUSTOTAL_API_KEY
        this.apiKey = process.env.VIRUSTOTAL_API_KEY || 'MOCK_API_KEY';
        this.baseUrl = 'https://www.virustotal.com/api/v3';
    }

    /**
     * Queries VirusTotal for a file hash.
     * @param {string} hash SHA-256 hash of the file
     * @returns {Object} Threat intel report containing malicious hits
     */
    async queryFileHash(hash) {
        // If we don't have a real API key, simulate a response for testing
        if (this.apiKey === 'MOCK_API_KEY') {
            console.log(`[ThreatIntel] Mocking VT response for hash: ${hash}`);
            // Mock an EICAR-like malicious response if the hash starts with 'bad'
            if (hash.startsWith('bad')) {
                return { isMalicious: true, enginesDetected: 45, classification: 'Trojan.Win32.Generic' };
            }
            return { isMalicious: false, enginesDetected: 0, classification: null };
        }

        try {
            const response = await axios.get(`${this.baseUrl}/files/${hash}`, {
                headers: { 'x-apikey': this.apiKey }
            });
            
            const stats = response.data.data.attributes.last_analysis_stats;
            const isMalicious = stats.malicious > 0;
            
            return {
                isMalicious,
                enginesDetected: stats.malicious,
                classification: response.data.data.attributes.meaningful_name || 'Unknown Malware'
            };
        } catch (error) {
            if (error.response && error.response.status === 404) {
                // File not found in VT database, this is fine, proceed to dynamic analysis
                return { isMalicious: false, enginesDetected: 0, classification: null };
            }
            console.error('[ThreatIntel] API Error:', error.message);
            return { isMalicious: false, enginesDetected: 0, classification: null }; // Fail open to dynamic analysis
        }
    }
}

module.exports = new ThreatIntelService();
