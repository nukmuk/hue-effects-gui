use mdns_sd::{ServiceDaemon, ServiceEvent};
use tauri::{State, Window};

use crate::{state_structs::AppStateStruct, HueBridge};

#[tauri::command]
pub fn discover_bridges(state: State<'_, AppStateStruct>, window: Window) -> Result<String, ()> {
    if state.0.lock().unwrap().searching {
        return Ok("already searching".to_string());
    }
    state.0.lock().unwrap().searching = true;

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_hue._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    println!("part 1");
    while state.0.lock().unwrap().searching {
        println!("part 2");
        match receiver.recv().unwrap() {
            ServiceEvent::ServiceResolved(info) => {
                let bridge = HueBridge {
                    address: info.get_addresses().iter().next().unwrap().to_string(),
                    name: info.get_fullname().to_string(),
                };
                println!("Resolved a new service: {:?}", &bridge);
                window.emit("bridgeFound", &bridge).unwrap();
            }
            other_event => {
                println!("Received other event: {:?}", &other_event);
            }
        }
    }
    println!("end");
    Ok("started discover".to_string())
}
