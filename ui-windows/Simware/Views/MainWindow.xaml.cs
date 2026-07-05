using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;

namespace Simware.Views
{
    public partial class MainWindow : Window
    {
        private DashboardControl dashboardView;
        private AnalysisWorkspaceControl workspaceView;
        private GlobalSearchControl searchView;

        public MainWindow()
        {
            InitializeComponent();
            
            dashboardView = new DashboardControl();
            workspaceView = new AnalysisWorkspaceControl();
            searchView = new GlobalSearchControl();

            // Default
            MainContent.Content = dashboardView;
        }

        private void Navigate_Click(object sender, RoutedEventArgs e)
        {
            Button btn = sender as Button;
            if (btn == null) return;

            string tag = btn.Tag.ToString();
            
            // Reset styles
            ResetNavStyle(NavDashboard);
            ResetNavStyle(NavWorkspace);
            ResetNavStyle(NavSearch);

            // Set active style
            btn.Background = (Brush)Application.Current.Resources["SurfaceLighterBrush"];

            if (tag == "Dashboard")
            {
                MainContent.Content = dashboardView;
            }
            else if (tag == "Workspace")
            {
                MainContent.Content = workspaceView;
            }
            else if (tag == "Search")
            {
                MainContent.Content = searchView;
            }
        }

        private void ResetNavStyle(Button btn)
        {
            btn.Background = Brushes.Transparent;
        }
    }
}
