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
            this.Loaded += DashboardControl_Loaded;
        } 

        private async void DashboardControl_Loaded(object sender, System.Windows.RoutedEventArgs e)
        {
            var stats = await _apiService.GetStatsAsync();
            if (stats != null)
            {
                TxtFilesAnalyzed.Text = stats.TotalAnalyzed.ToString();
                TxtThreatsBlocked.Text = stats.ThreatsBlocked.ToString();
                TxtAvgTime.Text = stats.AvgAnalysisTime;
                
                if (stats.ThreatsBlocked > 0)
                {
                    TxtThreatsTrend.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(248, 81, 73)); // Danger
                }

                RecentActivityList.ItemsSource = stats.RecentActivity;
            }
        }

        private void Export_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            if (sender is Button btn && btn.Tag is string id)
            {
                _apiService.ExportReport(id);
            }
        }

        private void ViewDetails_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            if (sender is Button btn && btn.Tag is string id)
            {
                // Navigate to workspace using main window
                if (System.Windows.Window.GetWindow(this) is MainWindow mainWindow)
                {
                    mainWindow.NavigateToWorkspace(id);
                }
            }
        }

        private async void ManualScan_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            var id = await _apiService.TriggerManualScanAsync("manual_upload_windows.exe");
            if (!string.IsNullOrEmpty(id))
            {
                if (System.Windows.Window.GetWindow(this) is MainWindow mainWindow)
                {
                    mainWindow.NavigateToWorkspace(id);
                }
            }
        }
    }
    
    public partial class AnalysisWorkspaceControl : UserControl 
    { 
        private ApiService _apiService = new ApiService();
        private string _activeAnalysisId = "";

        public AnalysisWorkspaceControl() 
        { 
            InitializeComponent(); 
            
            _apiService.OnAnalysisStart += () => {
                Dispatcher.Invoke(() => {
                    TxtLiveTelemetry.Text = "[Analysis Started]\n";
                    TxtStatus.Text = "ANALYZING";
                    BorderStatus.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(26, 210, 153, 34)); // Warning/Yellow bg
                    TxtStatus.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(210, 153, 34));
                });
            };

            _apiService.OnTelemetryReceived += (log) => {
                Dispatcher.Invoke(() => {
                    if (TxtLiveTelemetry.Text == "[Waiting for telemetry...]") TxtLiveTelemetry.Text = "";
                    TxtLiveTelemetry.Text += $"{log}\n";
                });
            };

            _apiService.OnAnalysisComplete += (decision, riskScore, explanation) => {
                Dispatcher.Invoke(() => {
                    TxtLiveTelemetry.Text += $"\n[Analysis Complete] Decision: {decision}, Risk: {riskScore}\nExplanation: {explanation}";
                    TxtStatus.Text = decision;
                    if (decision == "BLOCK") {
                        BorderStatus.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(26, 248, 81, 73));
                        TxtStatus.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(248, 81, 73));
                    } else {
                        BorderStatus.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(26, 35, 134, 54));
                        TxtStatus.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(35, 134, 54));
                    }
                });
            };

            _ = _apiService.ConnectWebSocketAsync();
        } 

        public async void SetActiveAnalysis(string id)
        {
            _activeAnalysisId = id;
            TxtLiveTelemetry.Text = "[Waiting for telemetry...]";
            
            var analysis = await _apiService.GetAnalysisAsync(id);
            if (analysis != null)
            {
                TxtActiveSandbox.Text = $"Active Sandbox: {analysis.FileName ?? "Unknown"} ({analysis.FileHash?.Substring(0, 12) ?? ""})";
                if (analysis.Status == "completed")
                {
                    TxtStatus.Text = analysis.FinalDecision ?? "COMPLETED";
                    if (analysis.FinalDecision == "BLOCK") {
                        BorderStatus.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(26, 248, 81, 73));
                        TxtStatus.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(248, 81, 73));
                    } else {
                        BorderStatus.Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(26, 35, 134, 54));
                        TxtStatus.Foreground = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromRgb(35, 134, 54));
                    }
                }
            }
        }
    }
    
    public partial class GlobalSearchControl : UserControl 
    { 
        private ApiService _apiService = new ApiService();
        public GlobalSearchControl() { InitializeComponent(); } 

        private async void TxtSearch_TextChanged(object sender, TextChangedEventArgs e)
        {
            var query = TxtSearch.Text.Trim();
            if (string.IsNullOrEmpty(query))
            {
                PlaceholderPanel.Visibility = System.Windows.Visibility.Visible;
                SearchResultsList.Visibility = System.Windows.Visibility.Collapsed;
                return;
            }

            var results = await _apiService.SearchAnalysesAsync(query);
            PlaceholderPanel.Visibility = System.Windows.Visibility.Collapsed;
            SearchResultsList.Visibility = System.Windows.Visibility.Visible;
            SearchResultsList.ItemsSource = results;
        }

        private void Export_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            if (sender is Button btn && btn.Tag is string id)
            {
                _apiService.ExportReport(id);
            }
        }

        private void ViewDetails_Click(object sender, System.Windows.RoutedEventArgs e)
        {
            if (sender is Button btn && btn.Tag is string id)
            {
                if (System.Windows.Window.GetWindow(this) is MainWindow mainWindow)
                {
                    mainWindow.NavigateToWorkspace(id);
                }
            }
        }
    }
}
