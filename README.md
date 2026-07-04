# SentinelX

> AI-Powered Pre-Execution Malware Simulation & Behavioral Analysis Platform

SentinelX is a cross-platform defensive cybersecurity platform that intercepts executables, scripts, documents, and other potentially dangerous files **before execution**, analyzes them in disposable isolated environments, correlates behavioral telemetry with AI, and recommends allow / quarantine / block actions with plain-English explanations.

**This is a defensive research project.** It is designed to detect and analyze suspicious behavior inside isolated sandboxes only — never to execute malicious actions outside controlled analysis environments.

## Supported Platforms

| Platform | Status |
|----------|--------|
| Linux    | Primary development target |
| macOS    | Supported via platform adapters |
| Windows  | Supported via platform adapters |

## Architecture Overview

```
User launches file
       ↓
Execution Interceptor  →  Static Analysis  →  Threat Intelligence
       ↓                        ↓                    ↓
   Risk Prediction  ←──────────────────────────────────
       ↓
Disposable Sandbox  →  User Simulation  →  Deception Environment
       ↓
Behavior Monitoring  →  Behavioral Correlation  →  AI Explainability
       ↓
Threat Classification  →  Risk Engine  →  Decision Engine
       ↓
Reporting (PDF / HTML / JSON)  +  Desktop UI
```

## Repository Structure

```
simware/
├── crates/                 # Rust workspace — core engine & services
├── services/               # Python ML microservice, optional workers
├── apps/                   # Tauri desktop application
├── docs/                   # Architecture, API, deployment, guides
├── deploy/                 # Docker, compose, Kubernetes manifests
├── migrations/             # PostgreSQL schema
├── rules/                  # YARA rules and detection templates
└── .github/workflows/      # CI/CD pipelines
```

## Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Core engine | Rust | Memory safety, cross-platform, performance |
| Desktop UI | Tauri + React + TypeScript | Native cross-platform shell, modern UX |
| API | Axum (REST) + Tonic (gRPC) | High-performance async services |
| ML / AI | Python (FastAPI) + ONNX | Flexible model iteration, portable inference |
| Database | PostgreSQL + SQLite | Server history + local agent cache |
| Observability | OpenTelemetry + tracing | Structured logs, metrics, traces |
| CI/CD | GitHub Actions | Cross-platform build & test matrix |

See [docs/architecture/TECH_STACK.md](docs/architecture/TECH_STACK.md) for full justification.

## Quick Start

### Prerequisites

- Rust 1.78+
- Node.js 20+
- Python 3.11+
- PostgreSQL 16+ (optional for server mode)
- Docker (optional)

### Build Core Engine

```bash
cargo build --workspace
cargo test --workspace
```

### Run API Server

```bash
cargo run -p sentinelx-api
# REST: http://localhost:8080
# OpenAPI: http://localhost:8080/api/v1/docs
```

### Run Desktop UI

```bash
cd apps/sentinelx-ui
npm install
npm run tauri dev
```

### Run ML Service

```bash
cd services/sentinelx-ml
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
uvicorn sentinelx_ml.main:app --reload --port 8090
```

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture Guide](docs/architecture/ARCHITECTURE.md) | System design & clean architecture layers |
| [API Specification](docs/api/openapi.yaml) | REST API OpenAPI 3.1 spec |
| [Database Schema](docs/database/SCHEMA.md) | Tables, indexes, retention |
| [Roadmap](docs/ROADMAP.md) | Milestones & sprint plan |
| [Testing Strategy](docs/TESTING.md) | Unit, integration, performance tests |
| [Deployment Guide](docs/deployment/DEPLOYMENT.md) | Docker, K8s, agent rollout |

## Security Principles

- **Least privilege** — agents run with minimal OS permissions
- **Secure defaults** — unknown files quarantined when confidence is high
- **Encrypted storage** — local quarantine and config at rest
- **Tamper detection** — integrity checks on agent binaries
- **Audit logging** — all decisions recorded immutably
- **Sandbox isolation** — network/filesystem/process isolation by default

## License

Apache-2.0 — See [LICENSE](LICENSE).

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).
