import Editor, { Monaco } from '@monaco-editor/react';
import { editor } from 'monaco-editor';
import { useRef, useState } from 'react';
import { GonkASMCompletionItemProvider } from '../language/GonkASMCompletionItemProvider';
import { GonkASMThemeData } from '../language/GonkASMThemeData';
import { GonkASMTokenProvider } from '../language/GonkASMTokenProvider';
import GonkASMGuide from '../guide/GonkASMGuide';
import GonkASMParser from '../parser/GonkASMParser';
import GonkBox from './GonkBox';
import { ProgramBinary } from 'gonkbox-emu';

function GonkASMEditor() {
	const monacoRef = useRef<Monaco>(null);
	const editorRef = useRef<editor.IStandaloneCodeEditor>(null);
	const parser = useRef<GonkASMParser>(null);

	const modelPath = "inmemory://model/GonkASMEditor";

	const code = String.raw`; defaultProgram.gonkASM
label msg				; refer to the next element as 'msg' in code
istr "hello world!\n"	; store a string in the binary

label start				; first instruction to run
move 1 bill				; bill = 1
move 2 charlie			; charlie = 2
add bill charlie 		; charlie += bill (charlie = 3)

comp bill charlie		; store bill in relation to charlie
move print microwave	; set our jump address to the print section
jumpne					; jump to microwave if bill != charlie
stop

label print
$PRINT msg				; print the string at the address
stop					; programs must always end with stop
`;

	let [guide, setGuide] = useState(false);

	let [programBinary, setProgramBinary] = useState<ProgramBinary | null>(null);

	function compile() {
		if (!parser.current)
			parser.current = new GonkASMParser();

		if (monacoRef.current && editorRef.current) {
			let model = editorRef.current.getModel();
			let value = model?.getValue();
			if (value) {
				parser.current.parse(value, monacoRef.current);
			}
		}

		let program = parser.current.getProgram();
		if (program)
			setProgramBinary(program);
	}

	function saveBinary() {
		if (programBinary) {
			let blob = new Blob([programBinary.get_binary_blob()], { type: "application" });
			let url = URL.createObjectURL(blob);

			let a = document.createElement("a");
			a.href = url;
			a.download = "program.gonk";
			a.click();

			URL.revokeObjectURL(url);
		}
	}

	function saveSource() {
		if (editorRef.current) {
			let model = editorRef.current.getModel();
			let value = model?.getValue();
			if (value) {
				let blob = new Blob([value], { type: "text/plain" });
				let url = URL.createObjectURL(blob);

				let a = document.createElement("a");
				a.href = url;
				a.download = "program.gonkASM";
				a.click();

				URL.revokeObjectURL(url);
			}
		}
	}

	function toggleGuide() {
		if (editorRef.current)
			editorRef.current.layout({} as editor.IDimension);
		setGuide(!guide);
	}

	function registerGonkASM(monaco: Monaco) {
		monaco.languages.register({ id: "gonkASM" });

		monaco.languages.setMonarchTokensProvider("gonkASM", GonkASMTokenProvider);

		monaco.languages.registerCompletionItemProvider("gonkASM", GonkASMCompletionItemProvider(monaco));

		monaco.editor.defineTheme("gonkTheme", GonkASMThemeData);
	}

	function handleEditorMount(editor: editor.IStandaloneCodeEditor, monaco: Monaco) {
		editorRef.current = editor;
		monacoRef.current = monaco;
	}

	return <>
		<div className="GonkASMEditor">
			<div id="EditorPane">
				<div id="GonkBoxASMTopbar" className="Topbar">
					<button id="CompileButton" onClick={compile}>Compile</button>
					<button id="SaveSourceButton" onClick={saveSource}>Save Source</button>
					<button id="SaveBinaryButton" onClick={saveBinary}>Save Binary</button>
					<button id="GuideButton" onClick={toggleGuide}>Guide</button>
				</div>
				{!guide && <div className="GonkASMEditorWindow">
					<Editor
						defaultLanguage="gonkASM"
						defaultValue={code}
						beforeMount={registerGonkASM}
						onMount={handleEditorMount}
						keepCurrentModel={true}
						theme="gonkTheme"
						path={modelPath}
						options={{
							minimap: {
								enabled: false
							}
						}}
					/>
				</div>}
				{guide && <GonkASMGuide />}
			</div>
			<div id="EmulatorPane">
				<GonkBox source={programBinary} />
			</div>
		</div>
		<div className="Footer">
			<span>Hello</span>
		</div>
	</>;
};

export default GonkASMEditor;
