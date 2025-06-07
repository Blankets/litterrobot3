use super::command::{Command, CommandRequest, CommandResponse, ResetDrawerRequest};
use super::robot::Robot;
use super::util;
use anyhow::Result;

pub struct Client {
    auth: super::auth::Client,
}

impl Client {
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        Ok(Self {
            auth: super::auth::Client::new(email, password).await?,
        })
    }

    pub async fn fetch_robots(&self) -> Result<Vec<Robot>> {
        let client = reqwest::Client::new();
        let user_id = self.auth.user_id().unwrap();
        let route = util::get_robots_url(&user_id);
        let bearer_token = self.auth.get_authorization().await.unwrap();
        let headers = util::get_api_headers(&bearer_token);
        let robots: Vec<Robot> = client
            .get(route)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;

        Ok(robots)
    }

    pub async fn cycle(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::Cycle).await
    }

    pub async fn power_off(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::PowerOff).await
    }

    pub async fn power_on(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::PowerOn).await
    }

    pub async fn lock_off(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::LockOff).await
    }

    pub async fn lock_on(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::LockOn).await
    }

    pub async fn night_light_off(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::NightLightOff).await
    }

    pub async fn night_light_on(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::NightLightOn).await
    }

    pub async fn sleep_mode_off(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::SleepModeOff).await
    }

    pub async fn sleep_mode_on(&self, robot_id: &str) -> Result<CommandResponse> {
        self.send_command(robot_id, Command::SleepModeOn).await
    }

    async fn send_command(&self, robot_id: &str, command: Command) -> Result<CommandResponse> {
        let client = reqwest::Client::new();
        let user_id = self.auth.user_id().unwrap();
        let url = util::get_dispatch_command_url(&user_id, &robot_id);
        let bearer_token = self.auth.get_authorization().await.unwrap();
        let headers = util::get_api_headers(&bearer_token);
        let request = CommandRequest::new(robot_id, command);
        let response: CommandResponse = client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn reset_drawer(&self, robot_id: &str, new_cycle_capacity: u64) -> Result<Robot> {
        let client = reqwest::Client::new();
        let user_id = self.auth.user_id().unwrap();
        let url = util::get_robot_by_id_url(&user_id, &robot_id);
        let bearer_token = self.auth.get_authorization().await.unwrap();
        let headers = util::get_api_headers(&bearer_token);
        let request = ResetDrawerRequest::new(new_cycle_capacity);
        let response: Robot = client
            .patch(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
