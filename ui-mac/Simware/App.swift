import SwiftUI

@main
struct SimwareApp: App {
    var body: some Scene {
        WindowGroup {
            DashboardView()
                .frame(minWidth: 1000, minHeight: 700)
                // Premium dark mode appearance
                .preferredColorScheme(.dark) 
        }
        .windowStyle(HiddenTitleBarWindowStyle())
    }
}
