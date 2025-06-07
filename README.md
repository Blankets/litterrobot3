# litterrobot3

This library seeks to enable users to interact with their litter robot via the [litter robot API](https://www.litter-robot.com/).

Enhancements and contributions are welcome.

> [!WARNING]
> The litter robot API is not publicly documented and breaking changes should be anticipated.

Add to your project with cargo

`cargo add litterrobot3`

## Examples

### Cycle all robots associated with your litter robot account

```rust
use anyhow::Result;
use dotenv::dotenv;
use litterrobot3::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let email = std::env::var("LR_EMAIL")?;
    let password = std::env::var("LR_PASSWORD")?;
    let client = Client::new(&email, &password).await?;

    let robots = client.fetch_robots().await?;

    for robot in &robots {
        let _ = client.cycle(&robot.litter_robot_id).await?;
    }

    Ok(())
}
```

### Print status of all robots associated with your litter robot account

```rust
use anyhow::Result;
use dotenv::dotenv;
use litterrobot3::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let email = std::env::var("LR_EMAIL")?;
    let password = std::env::var("LR_PASSWORD")?;
    let client = Client::new(&email, &password).await?;

    let robots = client.fetch_robots().await?;

    robots.iter().for_each(|r| println!("{}", r));

    Ok(())
}
```

### Monitor a robot and force a reboot if in a faulted state

```rust
use anyhow::Result;
use dotenv::dotenv;
use litterrobot3::{Client, UnitStatus};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let email = std::env::var("LR_EMAIL")?;
    let password = std::env::var("LR_PASSWORD")?;
    let client = Client::new(&email, &password).await?;
    let shared_client = Arc::new(Mutex::new(client));

    tokio::spawn({
        let client = Arc::clone(&shared_client);
        async move {
            loop {
                let mut seconds: u64 = 60;

                let robot = {
                    let locked = client.lock().await;
                    match locked.fetch_robots().await {
                        Ok(robots) if !robots.is_empty() => Some(robots[0].clone()),
                        Ok(_) => {
                            eprintln!("No robots found");
                            None
                        }
                        Err(e) => {
                            eprintln!("Fetch error: {e}");
                            None
                        }
                    }
                };

                if let Some(robot) = robot {
                    let locked = client.lock().await;
                    if unit_needs_power_cycle(robot.unit_status) {
                        let _ = locked.power_off(&robot.litter_robot_id).await;
                        tokio::time::sleep(Duration::from_secs(20)).await;
                        let _ = locked.power_on(&robot.litter_robot_id).await;
                        seconds -= 20;
                    }
                }

                tokio::time::sleep(Duration::from_secs(seconds)).await;
            }
        }
    });

    Ok(())
}

fn unit_needs_power_cycle(unit_status: UnitStatus) -> bool {
    match unit_status {
        UnitStatus::BR | UnitStatus::CSF => true,
        _ => false,
    }
}
```
