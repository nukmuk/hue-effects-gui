import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
// import "./App.css";
import Devices from "./components/Devices";
import Sidebar2 from "./components/Sidebar2";
import Bridges from "./components/Bridges";
import Effects from "./components/Effects";
import { Route, Routes } from "react-router-dom";
import { AppShell, Navbar, Header, MantineProvider, Box, ScrollArea } from '@mantine/core';
import { ThemeProvider } from "@emotion/react";

const bridgeIP = "192.168.1.21";

function App() {

	console.log("app test");

	const testDevice = [
		{
			metadata: {
				name: "test",
			},
			id: "testi",
			product_data: "asdf"
		},
	];

	const [devices, setDevices] = useState(testDevice);

	const url = `https://${bridgeIP}/clip/v2/resource/device`;

	useEffect(() => {
		console.log("fetching");
		invoke("fetch", { url: url }).then(body => {
			const bodyParsed = JSON.parse(body);
			const data = bodyParsed["data"];
			setDevices(data);
			console.log(data);
		});
	}, []);

	return (
		<AppShell
			navbar={< Navbar width={{ base: 300 }} height={"100%"} p="md" > {< Sidebar2 />}</Navbar >}
			styles={(theme) => ({
				main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0], height: "100vh" },
			})}
			padding={"0"}
		>

			<ScrollArea type="auto" p={"md"} style={{ height: "100%" }}>
				<Routes>
					<Route path="/" element={<Effects />} />
					<Route path="/lights" element={<Devices devices={devices} />} />
					<Route path="/bridges" element={<Bridges />} />
				</Routes>
			</ScrollArea >
		</AppShell >
	);
}

export default App;
