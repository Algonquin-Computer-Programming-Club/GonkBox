import { EmuError, GonkBoxEmu, ProgramBinary } from "gonkbox-emu";
import { useCallback, useEffect, useRef, useState } from "react";

type GonkBoxInputs = {
	source: ProgramBinary | null
}

function GonkBox({ source }: GonkBoxInputs) {
	const emulator = useRef<GonkBoxEmu>(new GonkBoxEmu());
	let [executing, setExecuting] = useState(false);
	let [running, setRunning] = useState(false);
	let [speed, setSpeed] = useState(500);
	let [bill, setBill] = useState(0);
	let [charlie, setCharlie] = useState(0);
	let [tim, setTim] = useState(0);
	let [paul, setPaul] = useState(0);
	let [microwave, setMicrowave] = useState(0);
	let [canada, setCanada] = useState(0);
	let [consoleOut, setConsoleOut] = useState("");
	let [consoleIn, setConsoleIn] = useState("");

	const step = useCallback(() => {
		let executing = emulator.current.is_executing();
		setExecuting(emulator.current.is_executing());
		if (!executing) {
			setRunning(false);
		}
		try {
			emulator.current.step();
			setBill(emulator.current.get_bill());
			setCharlie(emulator.current.get_charlie());
			setTim(emulator.current.get_tim());
			setPaul(emulator.current.get_paul());
			setMicrowave(emulator.current.get_microwave());
			setCanada(emulator.current.get_canada());

			let read = emulator.current.try_read();
			if (read) {
				let char = String.fromCharCode(read);
				setConsoleOut(out => out + char);
				console.log(read);
			}
			if (consoleIn.length > 0) {
				if (emulator.current.try_write(consoleIn[0].charCodeAt(0))) {
					setConsoleIn(c => c.substring(1));
				}
			}
		} catch (err) {
			if (err instanceof EmuError) {
				console.error(err);
			} else {
				console.error(`Unexpected Error: ${err}`);
				throw err;
			}
		}
	}, [consoleIn]);

	function stepButton() {
		if (!running) {
			step();
		} else {
			setRunning(false);
		}
	}

	function toggleRunning() {
		setRunning(!running);
	}

	useEffect(() => {
		if (source) {
			emulator.current.upload_program(source);
			setExecuting(emulator.current.is_executing());
			setBill(emulator.current.get_bill());
			setCharlie(emulator.current.get_charlie());
			setTim(emulator.current.get_tim());
			setPaul(emulator.current.get_paul());
			setMicrowave(emulator.current.get_microwave());
			setCanada(emulator.current.get_canada());
		}
	}, [source]);

	useEffect(() => {
		if (running) {
			const interval = setInterval(() => {
				step();
			}, speed);

			return () => clearInterval(interval);
		}
	}, [running, speed, step]);

	const onConsoleInput = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
		event.stopPropagation();
		event.preventDefault();
		if (event.key === "Enter") {
			setConsoleOut(c => c + "\n");
			setConsoleIn(c => c + "\n");
		} else if (event.key.length === 1) {
			setConsoleOut(c => c + event.key);
			setConsoleIn(c => c + event.key);
		}
	};

	return <div className="GonkBox">
		<div id="GonkBoxTopbar" className="Topbar">
			<button onClick={stepButton}>Step</button>
			<button onClick={toggleRunning}>{running ? "Stop" : "Run"}</button>
			<button onClick={() => setSpeed(500)}>500ms</button>
			<button onClick={() => setSpeed(250)}>250ms</button>
			<button onClick={() => setSpeed(100)}>100ms</button>
			<button onClick={() => setSpeed(25)}>25ms</button>
		</div>
		<div className="GonkBoxStateView">
			<p>Executing: {executing.toString()}</p>
			<table>
				<thead>
					<tr>
						<th>Register</th>
						<th>Value</th>
					</tr>
				</thead>
				<tbody>
					<tr>
						<td>Bill</td>
						<td>0x{bill.toString(16).padStart(4, '0')}</td>
					</tr>
					<tr>
						<td>Charlie</td>
						<td>0x{charlie.toString(16).padStart(4, '0')}</td>
					</tr>
					<tr>
						<td>Tim</td>
						<td>0x{tim.toString(16).padStart(4, '0')}</td>
					</tr>
					<tr>
						<td>Paul</td>
						<td>0x{paul.toString(16).padStart(4, '0')}</td>
					</tr>
					<tr>
						<td>Microwave</td>
						<td>0x{microwave.toString(16).padStart(4, '0')}</td>
					</tr>
					<tr>
						<td>Canada</td>
						<td>0x{canada.toString(16).padStart(4, '0')}</td>
					</tr>
				</tbody>
			</table>
			<h2>Console</h2>
			<textarea className="GonkBoxConsole" onKeyDownCapture={onConsoleInput} rows={10} cols={50} value={consoleOut}></textarea>
		</div>
	</div >
}

export default GonkBox;
