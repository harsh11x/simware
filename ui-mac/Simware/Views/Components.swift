import SwiftUI

// MARK: - Buttons
struct SimwareButton: View {
    let title: String
    let action: () -> Void
    var isPrimary: Bool = true
    
    var body: some View {
        Button(action: action) {
            Text(title)
                .font(SimwareTypography.bodyMd())
                .foregroundColor(isPrimary ? .white : .simwareTextSecondary)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .frame(minWidth: 100)
                .background(isPrimary ? Color.simwarePrimary : Color.simwareSurfaceLighter)
                .cornerRadius(8)
                .overlay(
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(isPrimary ? Color.clear : Color.simwareBorder, lineWidth: 1)
                )
        }
        .buttonStyle(PlainButtonStyle())
    }
}

// MARK: - Badges & Chips
struct StatusChip: View {
    let text: String
    let color: Color
    
    var body: some View {
        Text(text.uppercased())
            .font(SimwareTypography.labelMd())
            .foregroundColor(color)
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(color.opacity(0.1))
            .cornerRadius(8)
    }
}

// MARK: - Cards
struct SimwareCard<Content: View>: View {
    let content: Content
    let accentColor: Color?
    
    init(accentColor: Color? = nil, @ViewBuilder content: () -> Content) {
        self.accentColor = accentColor
        self.content = content()
    }
    
    var body: some View {
        HStack(spacing: 0) {
            if let accent = accentColor {
                Rectangle()
                    .fill(accent)
                    .frame(width: 4)
            }
            
            content
                .padding(16)
                .frame(maxWidth: .infinity, alignment: .leading)
        }
        .background(Color.simwareSurface)
        .cornerRadius(12)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color.simwareBorder, lineWidth: 1)
        )
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }
}

struct MetricCard: View {
    var title: String
    var value: String
    var trend: String
    var trendColor: Color
    var isAlert: Bool = false
    
    var body: some View {
        SimwareCard(accentColor: isAlert ? .simwareDanger : nil) {
            VStack(alignment: .leading, spacing: 8) {
                Text(title.uppercased())
                    .font(SimwareTypography.labelMd())
                    .foregroundColor(.simwareTextSecondary)
                
                Text(value)
                    .font(SimwareTypography.display())
                    .foregroundColor(.simwareTextPrimary)
                
                Text(trend)
                    .font(SimwareTypography.bodySm())
                    .foregroundColor(trendColor)
            }
        }
    }
}

// MARK: - Code Blocks
struct CodeBlock: View {
    let code: String
    
    var body: some View {
        Text(code)
            .font(SimwareTypography.codeMd())
            .foregroundColor(.simwareTextPrimary)
            .padding(16)
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(Color.black)
            .cornerRadius(8)
            .overlay(
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color.simwareBorder, lineWidth: 1)
            )
    }
}
