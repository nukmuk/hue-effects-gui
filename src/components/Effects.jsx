import React, { useState } from "react";
import RoomPreview from "./RoomPreview.jsx";
import { Button, Flex, NumberInput, SegmentedControl, Slider, Switch } from "@mantine/core";
import { invoke } from "@tauri-apps/api/tauri";
import { Prism } from "@mantine/prism";
import { useEffect } from "react";

export default function Effects(props) {

	const testLights = [
		{ position: { x: 1, y: 0.7, z: 0 } },
		{ position: { x: 0, y: 0.5, z: 0.5 } },
		{ position: { x: -1, y: 0.4, z: 0 } },
	]

	const [areas, setAreas] = useState([{}, { channels: testLights }]);
	const [streaming, setStreaming] = useState(false);
	const [testMode, setTestMode] = useState(JSON.parse(window.localStorage.getItem("testMode")));
	const [frequency, setFrequency] = useState(50);
	const [effect, setEffect] = useState("Rainbow");
	const [previewEnabled, setPreviewEnabled] = useState(JSON.parse(window.localStorage.getItem("previewEnabled")));

	useEffect(() => {
		if (!previewEnabled) setTestMode(false);
		window.localStorage.setItem("previewEnabled", previewEnabled);
		window.localStorage.setItem("testMode", testMode);
		invoke("set_test_mode", { enabled: testMode });
	}, [previewEnabled, testMode]); // todo add localStorage to scale, speed etc

	useEffect(() => {

	}, [frequency]); // todo add persistence to scale, speed etc

	const bridgeIP = "192.168.1.21";

	async function changeScale(e) {
		await invoke("edit_rainbow", { angle: -1, scale: e, speed: -1 });
	};
	async function changeSpeed(e) {
		await invoke("edit_rainbow", { angle: -1, scale: -1, speed: e });
	};
	async function changeAngle(e) {
		await invoke("edit_rainbow", { angle: e, scale: -1, speed: -1 });
	};

	async function getEntAreas() {
		console.log("get entertainment areas");
		let result = await invoke("fetch", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration` });
		const resultData = JSON.parse(result)["data"];
		setAreas(resultData);
		console.log(resultData);
	};

	async function startStream() {
		console.log("start stream");
		setStreaming(true);
		await invoke("start_stream", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765` });
	};

	async function stopStream() {
		console.log("stop stream");
		setStreaming(false);
		await invoke("stop_stream", { url: `https://${bridgeIP}/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765` });
	};

	function changeTestMode(enabled) {
		setTestMode(enabled);
	}

	function changeEffect(effect) {
		setEffect(effect);
		invoke("set_effect", { effect: effect });
	}

	return <>
		<Flex direction="column" gap={"md"}>
			<Flex gap={"xs"} align={"center"}>
				<Button variant="default" onClick={streaming ? stopStream : startStream}>{streaming ? "Stop" : "Start"}</Button>
				<Button onClick={getEntAreas}>Get Entertainment Areas</Button>
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
				onChange={changeEffect}
				data={[
					{ label: "Rainbow", value: "Rainbow" },
					{ label: "Flash", value: "Flash" },
					{ label: "Solid", value: "Solid" },
					{ label: "None", value: "None" }
				]}
			/>

			{previewEnabled ? <RoomPreview areas={areas} /> : null}

			Scale
			<Slider size={"lg"} color={"yellow"} defaultValue={1} step={0.01} max={10} onChange={e => changeScale(e)} />
			Speed
			<Slider size={"lg"} color={"indigo"} defaultValue={1} step={0.01} max={100} onChange={e => changeSpeed(e)} />
			Angle
			<Slider size={"lg"} color={"red"} max={360} disabled="true" onChange={e => changeAngle(e)} />

			<NumberInput label="Frequency" value={frequency} min={1} max={120} onChange={e => setFrequency(e)} />

			<Prism language="json" >
				{JSON.stringify(areas, null, 2)}
			</Prism>
		</Flex>
	</>
}