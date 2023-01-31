import React, { useEffect, useState } from "react";
import Device from "./Device.jsx";
import { Button, Card, Collapse, Flex, Paper, ScrollArea } from "@mantine/core";
import { ThemeContext } from "@emotion/react";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";


export default function Bridges(props) {

	const [bridges, setBridges] = useState([{ name: "test", address: "127.0.0.1" }, { name: "test2", address: "127.0.0.1" }]);


	useEffect(() => { // todo: discovery sometimes doesn't work in discovery mode
		const unlisten = appWindow.listen("bridgeFound", event => {
			const payload = event.payload;
			console.log("bridge found", payload);

			setBridges(prevBridges => [...prevBridges, payload]);
		});

		console.log("starting discovery");
		invoke("discover_bridges")

		return () => {
			console.log("stopping discovery");
			invoke("stop_discover");
			unlisten.then(f => f());
		}
	}, []);

	return <>
		<ScrollArea style={{ height: "100%", width: "100%" }}>
			<Flex
				// direction="row"
				gap="md"
				wrap={"wrap"}
			>
				{bridges.map((bridge) => <Device key={bridge.address} name={bridge.name} info={bridge.address} type="bridge" />)}
			</Flex>
		</ScrollArea>
	</>;
}