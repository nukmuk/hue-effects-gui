import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
// import './style.css';
import './fixScrollbar.css';
import { BrowserRouter } from 'react-router-dom';
import { MantineProvider } from '@mantine/core';

ReactDOM.createRoot(document.getElementById('root')).render(
	<React.StrictMode>
		<BrowserRouter>
			<MantineProvider
				theme={{
					colorScheme: 'dark', defaultRadius: 'md',
					// components: {
					// Button: {
					// defaultProps: {
					// 	variant: "default",
					// }

					// }
					// }
				}}
				withGlobalStyles
				withNormalizeCSS
			>
				<App />
			</MantineProvider>
		</BrowserRouter>,
	</React.StrictMode >
);
