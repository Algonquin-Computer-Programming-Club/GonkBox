import { EmuError, GonkBoxEmu, ProgramBinary } from "gonkbox-emu";
import { createRef, Ref, useCallback, useEffect, useRef, useState } from "react";

type GonkBoxInputs = {
	source: ProgramBinary | null
}

function GonkBox({ source }: GonkBoxInputs) {
	const emulator = useRef<GonkBoxEmu>(new GonkBoxEmu());
	const canvasRef: Ref<HTMLCanvasElement> = createRef();
	const consoleRef: Ref<HTMLTextAreaElement> = createRef();

	let [dims, setDims] = useState<{ x: number, y: number }>({ x: 0, y: 0 });
	let [memory, setMemory] = useState<Uint8Array | null>(null);
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
	let [memoryScroll, setMemoryScroll] = useState(0);

	useEffect(() => {
		if (!canvasRef.current) return;
		if (!memory) return;

		const canvas: HTMLCanvasElement = canvasRef.current;
		const ctx = canvas.getContext("2d")!;

		var scale = window.devicePixelRatio;
		canvas.width = canvas.offsetWidth * scale;
		canvas.height = canvas.offsetHeight * scale;

		ctx.fillStyle = "#1d2021";
		ctx.fillRect(0, 0, canvas.width, canvas.height);

		let cursor = Math.floor(memoryScroll);

		for (let i = cursor; i < 0x100; i++) {
			ctx.fillStyle = "#fbf1c7";
			ctx.font = `${12 * scale}pt Courier`;
			let offset = (i * 0x10).toString(16).padStart(4, '0');
			ctx.fillText(`0x${offset}`, 0, (i - cursor + 1) * 16 * scale);
			for (let j = 0; j < 16; j++) {
				let index = i * 0x10 + j;
				let byte = memory[index];
				let str = byte.toString(16).padStart(2, '0');
				let x = (j + 3) * 24 * scale;
				let y = (i - cursor + 1) * 16 * scale;
				if (index === emulator.current.get_paul()) {
					ctx.fillStyle = "#458588";
					ctx.fillRect(x - 2 * scale, y - 13 * scale, 24 * scale, 16 * scale);
					ctx.fillStyle = "#1d2021";
					ctx.fillText(str, x, y);
				} else if (index === emulator.current.get_microwave()) {
					ctx.fillStyle = "#8ec07c";
					ctx.fillRect(x - 2 * scale, y - 13 * scale, 24 * scale, 16 * scale);
					ctx.fillStyle = "#1d2021";
					ctx.fillText(str, x, y);
				} else {
					ctx.fillStyle = "#fbf1c7";
					ctx.fillText(str, x, y);
				}
			}
		}
	}, [canvasRef, memoryScroll, paul, microwave, memory, dims]);

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
			}
			if (consoleIn.length > 0) {
				if (emulator.current.try_write(consoleIn[0].charCodeAt(0))) {
					setConsoleIn(c => c.substring(1));
				}
			}
		} catch (err) {
			if (err instanceof EmuError) {
				emulator.current.stop_executing();
				let safeErr = err as EmuError;
				setConsoleOut(out => out + safeErr.format());
			} else {
				console.error(`Unexpected Error: ${err}`);
			}
		}

		setMemory(emulator.current.get_memory());
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

	function clearConsole() {
		setConsoleOut("");
		setConsoleIn("");
	}

	useEffect(() => {
		if (!consoleRef.current) return;

		consoleRef.current.scrollTop = consoleRef.current.scrollHeight;
		consoleRef.current.selectionStart = consoleOut.length;
		consoleRef.current.selectionEnd = consoleOut.length;
	}, [consoleRef, consoleOut]);

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

			setMemory(emulator.current.get_memory());
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

	const handleResize = () => {
		setDims({ x: window.innerWidth, y: window.innerHeight });
	};

	useEffect(() => {
		window.addEventListener("resize", handleResize);
		return () => window.removeEventListener("resize", handleResize);
	});

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

	const onConsoleFocus = (event: React.FocusEvent<HTMLTextAreaElement>) => {
		event.target.selectionStart = consoleOut.length;
		event.target.selectionEnd = consoleOut.length;
	};

	const onInputSpeed = (event: React.InputEvent<HTMLInputElement>) => {
		setSpeed((event.target as HTMLInputElement).valueAsNumber);
	};

	const onCanvasWheel = (event: React.WheelEvent) => {
		setMemoryScroll(s => Math.min(Math.max(s + event.deltaY * 0.01, 0), 0xfc));
	};

	return <div className="GonkBox">
		<div id="GonkBoxTopbar" className="Topbar">
			<button onClick={stepButton}>Step</button>
			<span className="TopbarDivider">|</span>
			<button onClick={toggleRunning}>{running ? "Stop" : "Run"}</button>
			<input className="Slider" name="Step Length (ms)" type="range" defaultValue={500} min={5} max={500} step={5} onInput={onInputSpeed}></input>
			<p className="Readout">{speed.toString(10)}ms</p>
			<span className="TopbarDivider">|</span>
			<button onClick={clearConsole}>Clear Console</button>
		</div>
		<div className="GonkBoxStateView">
			<div id="Readouts">
				<p>Executing: <span className="Readout">{executing.toString()}</span></p>
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
							<td className="Register">0x{bill.toString(16).padStart(4, '0')}</td>
						</tr>
						<tr>
							<td>Charlie</td>
							<td className="Register">0x{charlie.toString(16).padStart(4, '0')}</td>
						</tr>
						<tr>
							<td>Tim</td>
							<td className="Register">0x{tim.toString(16).padStart(4, '0')}</td>
						</tr>
						<tr>
							<td>Paul</td>
							<td className="Register">0x{paul.toString(16).padStart(4, '0')}</td>
						</tr>
						<tr>
							<td>Microwave</td>
							<td className="Register">0x{microwave.toString(16).padStart(4, '0')}</td>
						</tr>
						<tr>
							<td>Canada</td>
							<td className="Register">0x{canada.toString(16).padStart(4, '0')}</td>
						</tr>
					</tbody>
				</table>
				<canvas className="HexViewerCanvas" onWheelCapture={onCanvasWheel} ref={canvasRef}></canvas>
			</div>
			<textarea
				className="GonkBoxConsole"
				spellCheck="false"
				onFocus={onConsoleFocus}
				onKeyDownCapture={onConsoleInput}
				value={consoleOut}
				rows={10}
				ref={consoleRef}>
			</textarea>
		</div>
	</div >
}

export default GonkBox;
