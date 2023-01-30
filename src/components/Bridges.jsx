import React, { useEffect, useState } from "react";
import Device from "./Device.jsx";
import { Button, Collapse, Flex, Paper, ScrollArea } from "@mantine/core";
import { ThemeContext } from "@emotion/react";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";


export default function Bridges(props) {

	const [bridges, setBridges] = useState([]);


	useEffect(() => {
		const unlisten = appWindow.listen("bridgeFound", event => {
			const payload = event.payload;
			console.log("bridge found", payload);

			setBridges([...bridges, payload]);
		});

		console.log("starting discovery");
		invoke("discover_bridges").then(info => {
			console.log("discovery info:", info)
		});

		return () => {
			console.log("stopping discovery");
			unlisten.then(f => f());
			invoke("stop_discover");
		}
	}, []);

	return <>
		<ScrollArea style={{ height: "100%", width: "360px" }}>
			<Paper
				shadow="md"
				p="xs"
				radius="lg"
			>
				<Flex
					direction="column"
					gap="md"
				>
					{bridges.map((bridge) => <Device key={bridge.address} name={bridge.name} info={bridge.address} type="bridge" />)}
				</Flex>
			</Paper>
		</ScrollArea>
	</>;
}