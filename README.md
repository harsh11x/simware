# 🛡️ Simware: AI-Powered Pre-Execution Malware Simulation Platform

> **"Zero-Trust Execution. Total Behavioral Visibility."**

Simware is an advanced, cross-platform endpoint security and malware analysis platform. It intercepts every executable, script, or potentially dangerous file **before execution**, detonates it inside a highly isolated, ephemeral QEMU sandbox, and utilizes AI-driven behavioral analysis to generate a plain-English explanation of the file's intent. Based on this AI correlation, Simware recommends whether the file should be allowed, quarantined, or blocked—keeping endpoints secure from zero-day threats.

---

## 🎯 Mission Statement
To build a production-quality endpoint security solution that removes the guesswork from malware analysis. By combining native OS-level interception with ephemeral hardware virtualization and AI correlation, Simware empowers users to safely observe the behavior of any file before it ever touches their real host system.

---

## 🏗️ System Architecture & Blueprint

Simware operates on a distributed micro-architecture consisting of three core pillars:

### 1. The Interceptor Agent (Rust)
The frontline defense deployed on the host operating system.
* **macOS**: Leverages Apple's `EndpointSecurity` framework to hook `AUTH_EXEC` events, freezing process creation system-wide.
* **Windows** (Planned/Architecture): Leverages ETW (Event Tracing for Windows) and kernel-level filesystem minifilters.
* **Execution Block**: When a file is executed, the agent halts execution, calculates the SHA-256 hash, and queries the backend API.
* **Sandbox Orchestration**: If the file is unknown, the agent spins up a headless **QEMU Sandbox**, injecting the binary into a volatile virtual environment (Windows 10/11 or macOS).
* **Hibernation Snapshots**: The QEMU sandbox boots instantly using `-loadvm` snapshots, ensuring a pristine, cross-contamination-free environment for every detonation.

### 2. The Analysis Engine Backend (Node.js)
The brain of the operation that aggregates data and issues verdicts.
* **API Gateway**: A Node.js/Express server that ingests telemetry (Process Trees, Network Activity, File System modifications, Registry edits) from the Sandbox.
* **In-Memory Queue**: Manages the high-throughput asynchronous `AnalysisWorker` jobs without blocking the main event loop.
* **Database Layer**: Prisma ORM with SQLite for persistent storage of `Analysis`, `TelemetryEvent`, and `ThreatClassification` data.
* **AI Correlation**: Correlates the live stream of telemetry to generate an `AiRiskScore` (0.0 to 1.0) and issues a definitive `ALLOW` or `BLOCK` decision.

### 3. Native Desktop Workspaces (macOS & Windows)
The user-facing control centers for threat monitoring and manual analysis.
* **macOS (SwiftUI)**: A fully native, high-performance dashboard that provides real-time visibility into the sandbox telemetry, process trees, and AI verdicts.
* **Windows (WPF / .NET 8)**: A native C# application offering the exact same feature parity and design aesthetic for Windows enterprise environments.
* **Live Polling**: UIs fetch continuous status updates from the backend via asynchronous polling to visualize the threat landscape in real-time.

---

## 📂 Project Structure

```text
simware/
├── agent/                       # Rust-based native OS interceptor
│   ├── src/
│   │   ├── main.rs              # Entry point & OS Hooks (EndpointSecurity)
│   │   ├── api_client.rs        # Communicates with Node.js backend
│   │   └── sandbox_runner.rs    # QEMU orchestrator & hibernation logic
│   └── Cargo.toml
│
├── backend/                     # Node.js Analysis API & Database
│   ├── prisma/                  # Database Schema
│   │   └── schema.prisma        # Models: Analysis, TelemetryEvent, ThreatClassification
│   ├── server.js                # Express API Gateway (Uploads, Hashes, Polling)
│   ├── queue.js                 # In-memory Async Job Queue
│   ├── analysisWorker.js        # Background worker for AI Threat Scoring
│   ├── uploads/                 # Quarantined/Intercepted binary storage
│   └── package.json
│
├── ui-mac/                      # Native macOS Desktop App
│   └── Simware/
│       ├── App.swift            # SwiftUI Application Entry
│       ├── ApiService.swift     # URLSession Networking (Backend Polling)
│       ├── Color+Theme.swift    # Simware Design System (Tokens)
│       ├── Typography.swift     # Simware Typography System
│       └── Views/               # Dashboard, AnalysisWorkspace, GlobalSearch
│
├── ui-windows/                  # Native Windows Desktop App
│   ├── Simware.sln              # Visual Studio Solution
│   └── Simware/
│       ├── Simware.csproj       # WPF / .NET 8 Project config
│       ├── Models/              # C# Data Contracts (Analysis)
│       ├── Services/            # HttpClient Networking (ApiService.cs)
│       └── Views/               # XAML Controls (Dashboard, Workspace)
│
├── ui-designs/                  # Raw UI/UX design assets & prototypes
└── simware-core/                # Shared utilities or experimental modules
```

---

## 🚀 Getting Started

### Prerequisites
* **Node.js**: v18+ (for the backend)
* **Rust**: 1.70+ (for the agent)
* **QEMU**: (for the sandbox orchestrator)
* **Swift/Xcode**: (for macOS UI compilation)
* **.NET 8 SDK**: (for Windows UI compilation)

### 1. Start the Backend
```bash
cd backend
npm install
npx prisma db push
node server.js
```
*API runs on `http://localhost:8000`*

### 2. Run the Rust Agent (macOS)
*Note: Requires `sudo` for EndpointSecurity entitlements.*
```bash
cd agent
cargo build --release
sudo ./target/release/simware-agent
```

### 3. Launch the UI
**For macOS:**
```bash
cd ui-mac/Simware
swiftc App.swift Color+Theme.swift Typography.swift ApiService.swift Views/*.swift -o Simware -target arm64-apple-macosx11.0
./Simware
```

**For Windows:**
Open `ui-windows/Simware.sln` in Visual Studio and click "Start" (or run `dotnet run` inside the project folder on a Windows machine).

---

## 🔒 Security & Privacy
Simware processes potentially highly malicious binaries. The `QEMU` sandbox environments must be isolated from the host network (using `-net none` or strictly controlled NAT) to prevent worm propagation. By default, telemetry is processed entirely locally on the host machine.

## 📜 License
*Proprietary - Enterprise use only.*
