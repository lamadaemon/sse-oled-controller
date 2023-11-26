use std::sync::{Arc};
use log::{info, warn};
use tokio::sync::Mutex;
use crate::api::SSEngineAPI;
use crate::api_types::{C2SGameCreate, C2SGameEventCreate, EventData, EventValue};
use crate::types::{DataAccessorData, Icon, LineContent, LineData, MultiLineFrameData, ScreenData, ScreenFrameData, ScreenHandler, TextModifierData};

mod types;
mod api_types;
mod api;

macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("SSE OLED Clock 1.0 by lamadaemon");

    warn!("Early access software, robustness is not guaranteed!");

    let mut api = SSEngineAPI::new(None);
    api.setup(C2SGameCreate {
        game: "OLED_CLOCK".to_string(),
        game_display_name: Some("OLED Clock".to_string()),
        developer: Some("lamadaemon".to_string()),
        deinitialize_timer_length_ms: Some(15000u16),
    }).await.expect("Failed to setup SSE API");

    api.new_event_and_bind(C2SGameEventCreate {
        game: "OLED_CLOCK".to_string(),
        event: "TIME_UPDATE".to_string(),
        min_value: 0,
        max_value: 1,
        icon_id: Icon::Timer,
        value_optional: false,
    }, vec![ScreenHandler {
        device_type: "screened".to_string(),
        zone: "one".to_string(),    // Fixed value
        mode: "screen".to_string(), // Fixed value
        datas: vec![ScreenData::FrameData(ScreenFrameData::MultiLine(MultiLineFrameData {
            frame_modifiers_data: None,
            lines: vec![
                LineData {
                    content: LineContent::Text(TextModifierData {
                        has_text: true,
                        prefix: "Now  ".to_string(),
                        suffix: "".to_string(),
                        bold: false,
                        wrap: 0,
                    }),
                    data_accessor_data: Some(DataAccessorData {
                        context_frame_key: Some("curr_game".to_string()),
                        arg: None
                    }),
                },
                LineData {
                    content: LineContent::Text(TextModifierData {
                        has_text: true,
                        prefix: "Time  ".to_string(),
                        suffix: "".to_string(),
                        bold: false,
                        wrap: 0,
                    }),
                    data_accessor_data: None,
                },
                LineData {
                    content: LineContent::Text(TextModifierData {
                        has_text: true,
                        prefix: "/ lamadaemon /".to_string(),
                        suffix: "".to_string(),
                        bold: false,
                        wrap: 0,
                    }),
                    data_accessor_data: Some(DataAccessorData {
                        context_frame_key: Some("nullstr".to_string()),
                        arg: None
                    }),
                }
            ],
        }))],
    }]).await.expect("Failed to create TIME_UPDATE event");

    let game_name = Arc::new(Mutex::new("IDLE".to_string()));
    let ref_game_name = Arc::clone(&game_name);
    let end_task = Arc::new(Mutex::new(false));
    let ref_end_task = Arc::clone(&end_task);

    let update_task = tokio::spawn(async move {
        loop {
            if *ref_end_task.lock().await {
                info!("Stopping update task");
                api.done().await.expect("Failed to deinitialize SSE API");
                break;
            }


            api.trigger_event("TIME_UPDATE".to_string(), Some(EventData {
                value: EventValue::String(chrono::Local::now().format("%H:%M:%S").to_string()),
                frame: Some(map!{
                "curr_game".to_string() => EventValue::String((*ref_game_name.lock().await).clone()),
                "nullstr".to_string() => EventValue::String("".to_string())
            }),
            })).await.expect("Failed to trigger TIME_UPDATE event");

            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }

        return
    });

    info!("Setup complete, type 'help' for a list of commands");
    loop {
        // read commands from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // command syntax is <command> [args...]
        // split the input into a vector of strings and pop the command
        let mut args: Vec<&str> = input.split_whitespace().collect();
        let command = args.remove(0);

        match command {
            "help" => {
                info!("Available Commands:");
                info!("  set <game name> - Set the game name");
                info!("  idle - Set the game name to IDLE");
                info!("  exit - Exit the program");
            },
            "set" => {
                if args.len() == 0 {
                    println!("Failed to set game name: No game name provided");
                    continue;
                }

                let mut game_name = game_name.lock().await;
                *game_name = args.join(" ");
                info!("Update game name to {}", *game_name);

                drop(game_name);
            },
            "idle" => {
                let mut game_name = game_name.lock().await;
                *game_name = "IDLE".to_string();
                info!("Update game name to {}", *game_name);

                drop(game_name);
            },
            "exit" => {
                info!("Exiting...");
                let mut end_task_ref = end_task.lock().await;
                *end_task_ref = true;
                drop(end_task_ref);

                update_task.await.expect("Failed to await update task");
                break;
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }


}
