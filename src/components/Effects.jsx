import React, { useState } from "react";
import RoomPreview from "./RoomPreview.jsx";
import { Button, Flex, NumberInput, SegmentedControl, Slider, Switch } from "@mantine/core";
import { invoke } from "@tauri-apps/api/tauri";
import { Prism } from "@mantine/prism";
import { useEffect } from "react";
import RainbowControls from "./Effects/RainbowControls.jsx";
import SolidControls from "./Effects/SolidControls.jsx";
import { getFromStorage, useLocalStorage } from "../utils.js";
import exampleArea from "./exampleArea.js";

export default function Effects(props) {

	const testLights = [
		{ position: { x: 1, y: 0.7, z: 0 } },
		{ position: { x: 0, y: 0.5, z: 0.5 } },
		{ position: { x: -1, y: 0.4, z: 0 } },
	]

	const testArea = [{}, { channels: testLights }];
	const testArea2 = exampleArea;

	const [areas, setAreas] = useState(exampleArea);
	const [streaming, setStreaming] = useState(JSON.parse(sessionStorage.getItem("streaming")) || false);
	const [testMode, setTestMode] = useLocalStorage("testMode", true);
	const [frequency, setFrequency] = useLocalStorage("frequency", 50);
	const [effect, setEffect] = useLocalStorage("effect", "Rainbow");
	const [previewEnabled, setPreviewEnabled] = useLocalStorage("previewEnabled", true);

	useEffect(() => {
		// if (!previewEnabled) setTestMode(false);
		localStorage.setItem("previewEnabled", previewEnabled);
		localStorage.setItem("testMode", testMode);
		invoke("set_test_mode", { enabled: testMode });
	}, [previewEnabled, testMode]);

	useEffect(() => {
		console.log("set frequency", frequency);
		localStorage.setItem("frequency", frequency);
		invoke("edit_options", { options: { frequency } });
	}, [frequency]);

	useEffect(() => {
		localStorage.setItem("effect", effect);
		invoke("set_effect", { effect });
	}, [effect]);

	useEffect(() => {
		sessionStorage.setItem("streaming", streaming);
		if (!streaming) {
			console.log("stop stream");
			invoke("stop_stream", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765` });
		}
	}, [streaming]);

	function startStreaming() {
		console.log("starting streaming")
		setStreaming(true);
		invoke("start_stream", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765` });
	}

	const bridgeIP = "192.168.1.21";

	async function getEntAreas() {
		console.log("get entertainment areas");
		let result = await invoke("fetch", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration` });
		const resultData = JSON.parse(result)["data"];
		setAreas(resultData);
		sessionStorage.setItem("areas", JSON.stringify(resultData));
		console.log(resultData);
		console.log("stringify: ", JSON.stringify(resultData));
	};

	function changeTestMode(enabled) {
		setTestMode(enabled);
	}

	return <>
		<Flex direction="column" gap={"md"}>
			<Flex gap={"xs"} align={"center"}>
				<Button variant={streaming ? "outline" : "filled"} color={streaming ? "red" : "green"} onClick={streaming ? () => setStreaming(false) : startStreaming}>{streaming ? "Stop" : "Start"}</Button>
				<Button onClick={getEntAreas} >Get Entertainment Areas</Button>
				<Switch
					checked={previewEnabled}
					onChange={e => setPreviewEnabled(e => !e)}
					label="3D Room View"
					size="md"
				/>
				<Switch
					checked={testMode}
					onChange={e => {
						changeTestMode(e.currentTarget.checked);
					}}
					label="Test mode"
					size="md"
					disabled={streaming || !previewEnabled}
				/>

			</Flex>

			<SegmentedControl
				value={effect}
				onChange={setEffect}
				data={[
					{ label: "Rainbow", value: "Rainbow" },
					{ label: "Flash", value: "Flash" },
					{ label: "Solid", value: "Solid" },
					{ label: "None", value: "None" }
				]}
			/>

			{previewEnabled ? <RoomPreview areas={areas} /> : null}

			{effect === "Rainbow" ? <RainbowControls getFromStorage={getFromStorage} /> : null}
			{effect === "Solid" || effect === "Flash" ? <SolidControls /> : null}


			<NumberInput variant="filled" style={{ width: "120px" }} width={"sm"} label="Frequency" value={frequency} min={1} max={120} onChange={setFrequency} />

			<Prism language="json" >
				{JSON.stringify(areas, null, 2)}
			</Prism>
		</Flex>
	</>
}