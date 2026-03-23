import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import GonkASMEditor from './components/GonkASMEditor';

const root = ReactDOM.createRoot(
	document.getElementById('root') as HTMLElement
);

root.render(
	<React.StrictMode>
		<GonkASMEditor />
	</React.StrictMode>
);
