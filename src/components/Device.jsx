import { ThemeProvider } from "@emotion/react";
import { Button, Code, Collapse, Flex, MantineProvider, Paper, Text } from "@mantine/core";
import React, { useState } from "react";
import { Prism } from "@mantine/prism";
// import "./Device.css";


export default function Device(props) {

	// const code = JSON.stringify(props.info, "", " ");
	const code = JSON.stringify(props.info, null, 2);
	const [opened, setOpened] = useState(false);

	if (props.type === "bridge") {
		return <>
			<Paper
				shadow="md"
				p="xs"
				radius="lg"
				style={{ minWidth: "300px" }}
			>
				<Flex direction="column" p={"4px"}>
					<Text>{props.name}</Text>
					<Text color={"gray.6"}>{props.info}</Text>
					<Flex justify={"flex-end"}>
						<Button>Connect</Button>
					</Flex>
				</Flex>
			</Paper>
		</>
	} else {
		return <>
			<Button my="12px" onClick={() => setOpened(o => !o)}>{props.name}</Button>
			<Collapse in={opened}>
				<Prism style={{ width: "330px" }} language="json">{code}</Prism>
			</Collapse>
		</>
	}
}
