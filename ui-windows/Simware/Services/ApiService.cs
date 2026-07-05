using System;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;
using Simware.Models;

namespace Simware.Services
{
    public class ApiService
    {
        private readonly HttpClient _client;
        private const string BaseUrl = "http://localhost:8000/api/v1";

        public ApiService()
        {
            _client = new HttpClient();
        }

        public async Task<Analysis> GetAnalysisAsync(string analysisId)
        {
            try
            {
                var response = await _client.GetAsync($"{BaseUrl}/analysis/{analysisId}");
                response.EnsureSuccessStatusCode();
                
                var content = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<Analysis>(content);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error fetching analysis: {ex.Message}");
                return null;
            }
        }
    }
}
