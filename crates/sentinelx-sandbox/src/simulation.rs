use async_trait::async_trait;
use sentinelx_core::{Result, UserSimulator};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct ActivityProfile {
    pub name: String,
    pub mouse_movements: u32,
    pub keystrokes: u32,
    pub idle_periods_ms: u64,
    pub open_documents: u32,
}

impl ActivityProfile {
    pub fn office_worker() -> Self {
        Self {
            name: "office_worker".into(),
            mouse_movements: 20,
            keystrokes: 50,
            idle_periods_ms: 2000,
            open_documents: 2,
        }
    }

    pub fn passive() -> Self {
        Self {
            name: "passive".into(),
            mouse_movements: 5,
            keystrokes: 10,
            idle_periods_ms: 5000,
            open_documents: 0,
        }
    }
}

pub struct DefaultUserSimulator;

#[async_trait]
impl UserSimulator for DefaultUserSimulator {
    async fn run_profile(&self, instance_id: &str, profile: &str) -> Result<()> {
        let activity = match profile {
            "office_worker" => ActivityProfile::office_worker(),
            "passive" | _ => ActivityProfile::passive(),
        };

        tracing::info!(
            instance_id,
            profile = activity.name,
            "simulating user activity"
        );

        for _ in 0..activity.mouse_movements {
            sleep(Duration::from_millis(100)).await;
        }
        sleep(Duration::from_millis(activity.idle_periods_ms)).await;

        Ok(())
    }
}
