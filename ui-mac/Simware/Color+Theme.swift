import SwiftUI

extension Color {
    static let simwareBackground = Color(hex: "#0F1115")
    static let simwareSurface = Color(hex: "#1A1D23")
    static let simwareSurfaceLighter = Color(hex: "#242830")
    
    static let simwarePrimary = Color(hex: "#3B82F6")
    
    static let simwareSuccess = Color.green // Assuming standard green is fine or hex
    static let simwareWarning = Color.orange
    static let simwareDanger = Color.red
    
    static let simwareBorder = Color(hex: "#2D333B")
    
    static let simwareTextPrimary = Color(hex: "#F9FAFB")
    static let simwareTextSecondary = Color(hex: "#9CA3AF")
}

extension Color {
    init(hex: String) {
        let hex = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
        var int: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&int)
        let a, r, g, b: UInt64
        switch hex.count {
        case 3: // RGB (12-bit)
            (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
        case 6: // RGB (24-bit)
            (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
        case 8: // ARGB (32-bit)
            (a, r, g, b) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
        default:
            (a, r, g, b) = (1, 1, 1, 0)
        }

        self.init(
            .sRGB,
            red: Double(r) / 255,
            green: Double(g) / 255,
            blue:  Double(b) / 255,
            opacity: Double(a) / 255
        )
    }
}
