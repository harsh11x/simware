use clap::Parser;
use sentinelx_agent::AnalysisPipeline;
use sentinelx_core::ReportFormat;
use sentinelx_reporting::DefaultReportGenerator;
use sentinelx_storage::SqliteAnalysisStore;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "sentinelx-agent", about = "SentinelX pre-execution analysis agent")]
struct Cli {
    /// File to analyze
    path: PathBuf,

    /// Skip sandbox execution (static analysis only)
    #[arg(long)]
    static_only: bool,

    /// Export report to directory
    #[arg(long)]
    export_dir: Option<PathBuf>,

    /// SQLite database path
    #[arg(long, default_value = "sqlite:data/sentinelx.db")]
    database: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("sentinelx=info".parse()?))
        .init();

    let cli = Cli::parse();
    let store = Arc::new(SqliteAnalysisStore::new(&cli.database).await?);
    let pipeline = AnalysisPipeline::new()
        .with_store(store)
        .skip_sandbox(cli.static_only);

    let (ctx, report) = pipeline.analyze_file(&cli.path).await?;

    println!("Session: {}", ctx.session_id());
    if let Some(decision) = &ctx.decision {
        println!("Decision: {:?}", decision.action);
        println!("Risk Score: {:.2}", decision.risk.score);
        println!("Rationale: {}", decision.rationale);
    }

    if let Some(dir) = cli.export_dir {
        std::fs::create_dir_all(&dir)?;
        let generator = DefaultReportGenerator;
        for (format, ext) in [
            (ReportFormat::Json, "json"),
            (ReportFormat::Html, "html"),
        ] {
            let bytes = generator.export(&report, format).await?;
            let path = dir.join(format!("{}.{ext}", ctx.session_id()));
            std::fs::write(&path, bytes)?;
            println!("Exported: {}", path.display());
        }
    }

    Ok(())
}
