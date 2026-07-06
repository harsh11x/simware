mod interceptor;
mod sandbox;
mod telemetry;
mod api_client;
mod quarantine;

use interceptor::{ExecutionInterceptor, NativeInterceptor};
use sandbox::{SandboxOrchestrator, QemuSandbox};
use api_client::BackendClient;
use std::env;

fn main() {
    println!("Simware Agent Starting...");
    println!("Initializing native OS hooks (EndpointSecurity)...");

    let interceptor = NativeInterceptor;
    
    if let Err(e) = interceptor.start() {
        eprintln!("Failed to start interceptor: {}", e);
        return;
    }

    // For the sake of this prototype, if a file path is passed as an argument, we will "intercept" it.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let target_file = &args[1];
        println!("\n[Interceptor] Detected execution attempt for: {}", target_file);
        println!("[Interceptor] BLOCKING execution pending analysis...");

        // 1. Submit to Backend
        let client = BackendClient::new("http://localhost:8000");
        println!("[API] Submitting file to backend for analysis...");
        
        match client.submit_for_analysis(target_file) {
            Ok(analysis_id) => {
                println!("[API] Analysis queued. ID: {}", analysis_id);
                
                // 2. Prepare Sandbox locally for deep behavioral analysis
                let qemu = QemuSandbox::new("/opt/simware/images/win10_base.qcow2", "clean_snapshot");
                if let Err(e) = qemu.setup(target_file) {
                    eprintln!("[Sandbox] Setup failed: {}", e);
                } else {
                    if let Err(e) = qemu.execute() {
                        eprintln!("[Sandbox] Execution failed: {}", e);
                    }
                    let _ = qemu.cleanup();
                }

                // 3. Wait for final verdict from backend (AI correlated)
                println!("[API] Waiting for AI behavioral correlation verdict...");
                match client.wait_for_verdict(&analysis_id) {
                    Ok(decision) => {
                        println!("\n=================================");
                        println!("FINAL VERDICT: {}", decision);
                        println!("=================================\n");
                        
                        if decision == "BLOCK" {
                            println!("[Interceptor] Permanently blocking execution of {}", target_file);
                            if let Err(e) = quarantine::quarantine_file(target_file) {
                                eprintln!("[Quarantine] Failed to quarantine file: {}", e);
                            }
                        } else {
                            println!("[Interceptor] Allowing execution of {}", target_file);
                        }
                    },
                    Err(e) => eprintln!("[API] Failed to get verdict: {}", e),
                }
            },
            Err(e) => {
                eprintln!("[API] Failed to submit analysis: {}", e);
            }
        }
    } else {
        println!("Usage: simware-agent <path_to_executable_to_intercept>");
    }

    if let Err(e) = interceptor.stop() {
        eprintln!("Failed to stop interceptor: {}", e);
    }
}
