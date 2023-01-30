import { invoke } from "@tauri-apps/api/tauri";
// import "./sidebar.css";

export default function Sidebar() {

	return <div className="sidebar">
		<button type="button" onClick={startStream}>Start</button>
		<button type="button" onClick={discoverBridges}>Discover</button>
		<button type="button" onClick={stopDiscover}>Stop Discover</button>
	</div>;
}

const bridgeIP = "192.168.1.21";
const username = "jFH2gdRLevNsZe0pvfsx6aT8hvrDhvRrfIGOVv8i";
const psk = "520365836F859C7AEF2AC64D8AC500D4";

async function startStream() {
	console.log("start stream");
	await invoke("start_stream", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765` });

};

async function discoverBridges() {
	console.log("start discovery");
	let result = await invoke("discover_bridges");
	console.log(result);
};

async function stopDiscover() {
	console.log("stop discovery");
	let result = await invoke("stop_discover");
	console.log(result);
};
