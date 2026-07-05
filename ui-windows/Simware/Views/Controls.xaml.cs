using System;
using System.Windows.Controls;
using System.Windows.Threading;
using Simware.Services;
using Simware.Models;

namespace Simware.Views
{
    public partial class DashboardControl : UserControl 
    { 
        private ApiService _apiService = new ApiService();
        public DashboardControl() 
        { 
            InitializeComponent(); 
            // In a full implementation, we'd fetch the latest stats.
        } 
    }
    
    public partial class AnalysisWorkspaceControl : UserControl 
    { 
        private ApiService _apiService = new ApiService();
        private DispatcherTimer _pollTimer;
        private string _activeAnalysisId = "1f2780d9-1307-46ba-b110-a79d407f4392"; // Hardcoded for demo/testing

        public AnalysisWorkspaceControl() 
        { 
            InitializeComponent(); 
            
            _pollTimer = new DispatcherTimer();
            _pollTimer.Interval = TimeSpan.FromSeconds(2);
            _pollTimer.Tick += async (s, e) => {
                var analysis = await _apiService.GetAnalysisAsync(_activeAnalysisId);
                if (analysis != null)
                {
                    // Update UI elements based on analysis state
                    // This links the XAML dashboard directly to the backend telemetry
                }
            };
            _pollTimer.Start();
        } 
    }
    
    public partial class GlobalSearchControl : UserControl 
    { 
        public GlobalSearchControl() { InitializeComponent(); } 
    }
}
