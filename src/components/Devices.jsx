import React from "react";
import Device from "./Device.jsx";
import { Collapse, Flex, Paper, ScrollArea } from "@mantine/core";
import { ThemeContext } from "@emotion/react";
// import "./Devices.css";

export default function Devices(props) {
	const devices = props.devices.map((device) => <Device name={device.metadata.name} key={device.id} info={device.product_data} />);

	console.log(devices);

	return <>
		<ScrollArea style={{ height: "100%", width: "360px" }}>
			<Paper
				shadow="md"
				px="xs"
				py="4px"
				radius="xl"
			>
				<Flex
					direction="column"
					gap="0"
					m="4px"
				>
					{devices}
				</Flex>
			</Paper>
		</ScrollArea>
	</>;
}
