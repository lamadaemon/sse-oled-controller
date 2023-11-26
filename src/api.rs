use serde::{Deserialize, Serialize};
use log::debug;
use crate::api_types::{C2SGameCreate, C2SGameEventBind, C2SGameEventCreate, C2SGameEventRemove, C2SGameRemove, C2SHeartBeat, C2STriggerEvent, EventData};
use crate::types::ScreenHandler;

pub struct SSEngineAPI {
    server: String,
    http: reqwest::Client,
    game: Option<String>,
    heat_beat_task: Option<tokio::task::JoinHandle<Result<(), reqwest::Error>>>,
    update_interval: Option<u16>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreProperties {
    address: String
}

impl SSEngineAPI {
    #[cfg(target_os = "windows")]
    const DEFAULT_CORE_PROP_LOCATION: &'static str = "%PROGRAMDATA%/SteelSeries/SteelSeries Engine 3/coreProps.json";
    #[cfg(target_os = "macos")]
    const DEFAULT_CORE_PROP_LOCATION: &'static str = "/Library/Application Support/SteelSeries Engine 3/coreProps.json";

    pub async fn setup(&mut self, game: C2SGameCreate) -> Result<(), reqwest::Error> {
        let res = self.http.post(&self.endpoint("game_metadata"))
            .json(&game)
            .send()
            .await;

        debug!("Setup response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        self.game = Some(game.game.to_string());
        self.update_interval = game.deinitialize_timer_length_ms;

        self.launch_heart_beat_task();

        Ok(())
    }

    pub fn launch_heart_beat_task (&mut self) {
        let body = C2SHeartBeat {
            game: self.game.clone().unwrap()
        };
        let endpoint = self.endpoint("game_heartbeat");
        let interval = self.update_interval.unwrap_or(10000) as u64;

        let task = tokio::spawn(async move {
            let http = reqwest::Client::new();

            loop {
                let res = http.post(&endpoint)
                    .json(&body)
                    .send()
                    .await;

                debug!("Heartbeat response: {:?}", res);

                if res.is_err() {
                    let err = res.err().unwrap();
                    debug!("Error: {:?}", err);
                    return Err(err);
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
            }
        });

        self.heat_beat_task = Some(task);
    }

    pub async fn new_event(&self, event: C2SGameEventCreate) -> Result<(), reqwest::Error> {
        debug!("Creating a new event: {:?}", serde_json::to_string(&event));

        let res = self.http.post(&self.endpoint("register_game_event"))
            .json(&event)
            .send()
            .await;

        debug!("New event response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        Ok(())
    }

    pub async fn new_event_and_bind(&self, event: C2SGameEventCreate, handlers: Vec<ScreenHandler>) -> Result<(), reqwest::Error> {
        self.new_event(event.clone()).await?;
        self.bind_event(C2SGameEventBind {
            game: event.game,
            event: event.event,
            min_value: event.min_value,
            max_value: event.max_value,
            icon_id: event.icon_id,
            handlers
        }).await?;

        Ok(())
    }

    pub async fn bind_event(&self, binding: C2SGameEventBind) -> Result<(), reqwest::Error> {
        debug!("Binding event: {:?}", serde_json::to_string(&binding));

        let res = self.http.post(&self.endpoint("bind_game_event"))
            .json(&binding)
            .send()
            .await;

        debug!("Bind event response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        Ok(())
    }

    pub async fn trigger_event(&self, event: String, data: Option<EventData>) -> Result<(), reqwest::Error> {
        let body = &C2STriggerEvent {
            game: self.game.clone().unwrap(),
            event,
            data
        };

        debug!("Triggering event {:?} ", serde_json::to_string(&body));
        let res = self.http.post(&self.endpoint("game_event"))
            .json(body)
            .send()
            .await;

        debug!("Trigger event response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        Ok(())
    }

    pub async fn remove_event(&self, event: C2SGameEventRemove) -> Result<(), reqwest::Error> {
        let res = self.http.post(&self.endpoint("remove_game_event"))
            .json(&event)
            .send()
            .await;

        debug!("Remove event response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        Ok(())
    }

    pub async fn done(&self) -> Result<(), reqwest::Error> {
        self.heat_beat_task.as_ref().unwrap().abort();

        let res = self.http.post(&self.endpoint("remove_game"))
            .json(&C2SGameRemove {
                game: self.game.clone().unwrap()
            })
            .send()
            .await;

        debug!("Done response: {:?}", res);

        if res.is_err() {
            let err = res.err().unwrap();
            debug!("Error: {:?}", err);
            return Err(err);
        }

        Ok(())
    }

    pub fn new(server: Option<&str>) -> SSEngineAPI {
        let server_url = if server.is_none() {
            let mut path = SSEngineAPI::DEFAULT_CORE_PROP_LOCATION.to_string();
            #[cfg(target_os = "windows")] {
                    let app_data = std::env::var("PROGRAMDATA").expect("No PROGRAMDATA directory");
                    path = path.replace("%PROGRAMDATA%", &app_data);
            }
            let core_props = std::fs::read_to_string(path).unwrap();
            let core_props: CoreProperties = serde_json::from_str(&core_props).unwrap();
            debug!("Core props: {:?}", core_props);
            debug!("Readed api url is: {}", core_props.address);

            core_props.address
        } else {
            server.unwrap().to_string()
        };

        return SSEngineAPI {
            server: server_url,
            http: reqwest::Client::new(),
            game: None,
            heat_beat_task: None,
            update_interval: None
        }
    }

    fn endpoint(&self, ep: &str) -> String {
        format!("http://{}/{}", self.server, ep)
    }
}
