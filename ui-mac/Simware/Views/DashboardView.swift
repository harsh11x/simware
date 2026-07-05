import SwiftUI

struct DashboardView: View {
    @State private var selectedTab: String? = "Dashboard"
    
    var body: some View {
        NavigationView {
            // Custom Sidebar
            VStack(alignment: .leading, spacing: 0) {
                // Header
                HStack {
                    Image(systemName: "shield.checkered")
                        .foregroundColor(.simwarePrimary)
                        .font(.system(size: 24, weight: .bold))
                    Text("Simware")
                        .font(SimwareTypography.h2())
                        .foregroundColor(.simwareTextPrimary)
                }
                .padding(.horizontal, 24)
                .padding(.vertical, 32)
                
                // Navigation Links
                VStack(spacing: 8) {
                    SidebarItem(icon: "chart.bar.fill", title: "Dashboard", isSelected: selectedTab == "Dashboard") {
                        selectedTab = "Dashboard"
                    }
                    SidebarItem(icon: "magnifyingglass", title: "Analysis Workspace", isSelected: selectedTab == "Workspace") {
                        selectedTab = "Workspace"
                    }
                    SidebarItem(icon: "doc.text.magnifyingglass", title: "Global Search", isSelected: selectedTab == "Search") {
                        selectedTab = "Search"
                    }
                    SidebarItem(icon: "lock.shield", title: "Quarantine", isSelected: selectedTab == "Quarantine") {
                        selectedTab = "Quarantine"
                    }
                }
                .padding(.horizontal, 16)
                
                Spacer()
                
                // Agent Status
                HStack(spacing: 12) {
                    Circle()
                        .fill(Color.simwareSuccess)
                        .frame(width: 8, height: 8)
                    Text("Agent Active • v1.0.4")
                        .font(SimwareTypography.bodySm())
                        .foregroundColor(.simwareTextSecondary)
                }
                .padding(24)
            }
            .frame(width: 240, alignment: .leading)
            .background(Color.simwareSurface)
            
            // Main Content Area
            ZStack {
                Color.simwareBackground.ignoresSafeArea()
                
                if selectedTab == "Dashboard" {
                    DashboardContent()
                } else if selectedTab == "Workspace" {
                    AnalysisWorkspaceView()
                } else if selectedTab == "Search" {
                    GlobalSearchView()
                } else {
                    Text("Coming Soon")
                        .font(SimwareTypography.h1())
                        .foregroundColor(.simwareTextSecondary)
                }
            }
        }
        .navigationViewStyle(DoubleColumnNavigationViewStyle())
        .background(Color.simwareBackground)
    }
}

struct SidebarItem: View {
    let icon: String
    let title: String
    let isSelected: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.system(size: 16))
                    .foregroundColor(isSelected ? .simwarePrimary : .simwareTextSecondary)
                    .frame(width: 24)
                
                Text(title)
                    .font(SimwareTypography.bodyMd())
                    .foregroundColor(isSelected ? .simwareTextPrimary : .simwareTextSecondary)
                
                Spacer()
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 10)
            .background(isSelected ? Color.simwareSurfaceLighter : Color.clear)
            .cornerRadius(8)
        }
        .buttonStyle(PlainButtonStyle())
    }
}

struct DashboardContent: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 32) {
                // Header
                HStack {
                    Text("Overview")
                        .font(SimwareTypography.display())
                        .foregroundColor(.simwareTextPrimary)
                    
                    Spacer()
                    
                    SimwareButton(title: "Export Report", action: {})
                }
                
                // Metrics
                HStack(spacing: 24) {
                    MetricCard(title: "Files Analyzed", value: "1,248", trend: "↑ 12% vs last week", trendColor: .simwareTextSecondary)
                    MetricCard(title: "Threats Blocked", value: "14", trend: "↑ 3 new threats", trendColor: .simwareDanger, isAlert: true)
                    MetricCard(title: "Avg Analysis Time", value: "1.2s", trend: "Optimal", trendColor: .simwareSuccess)
                }
                
                // Recent Activity
                VStack(alignment: .leading, spacing: 16) {
                    Text("Recent Simulations")
                        .font(SimwareTypography.h2())
                        .foregroundColor(.simwareTextPrimary)
                    
                    SimwareCard {
                        VStack(spacing: 0) {
                            ActivityRow(filename: "installer_v2.exe", hash: "a2b4...9f01", status: "Clean", color: .simwareSuccess)
                            Divider().background(Color.simwareBorder)
                            ActivityRow(filename: "invoice_update.pdf.exe", hash: "f7c9...33d2", status: "Malicious", color: .simwareDanger)
                            Divider().background(Color.simwareBorder)
                            ActivityRow(filename: "npm_install_script.sh", hash: "b88a...11e4", status: "Suspicious", color: .simwareWarning)
                        }
                    }
                }
                
                Spacer()
            }
            .padding(32)
        }
    }
}

struct ActivityRow: View {
    let filename: String
    let hash: String
    let status: String
    let color: Color
    
    var body: some View {
        HStack {
            Image(systemName: "doc.fill")
                .foregroundColor(.simwareTextSecondary)
            
            VStack(alignment: .leading, spacing: 4) {
                Text(filename)
                    .font(SimwareTypography.bodyMd())
                    .foregroundColor(.simwareTextPrimary)
                Text(hash)
                    .font(SimwareTypography.codeMd())
                    .foregroundColor(.simwareTextSecondary)
            }
            
            Spacer()
            
            StatusChip(text: status, color: color)
            
            SimwareButton(title: "View Details", action: {}, isPrimary: false)
                .padding(.leading, 16)
        }
        .padding(.vertical, 12)
    }
}
