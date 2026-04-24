import Editor, { Monaco } from '@monaco-editor/react';
import { editor } from 'monaco-editor';
import { useRef, useState } from 'react';
import { GonkASMCompletionItemProvider } from '../language/GonkASMCompletionItemProvider';
import { GonkASMThemeData } from '../language/GonkASMThemeData';
import { GonkASMTokenProvider } from '../language/GonkASMTokenProvider';
import GonkASMEditorHeader from './GonkASMEditorHeader';
import GonkASMGuide from '../guide/GonkASMGuide';
import GonkASMParser from '../parser/GonkASMParser';

function GonkASMEditor() {
	const editorRef = useRef<editor.IStandaloneCodeEditor>(null);
	const monacoRef = useRef<Monaco>(null);
	const parser = useRef<GonkASMParser>(null);

	let [guide, setGuide] = useState(false);

	function compile() {
		if (!parser.current)
			parser.current = new GonkASMParser();

		if (editorRef.current && monacoRef.current)
			parser.current.parse(editorRef.current.getValue(), monacoRef.current);
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

	const code = [
		".label msg				; the strings name is msg when we're coding",
		"istr \"hello world!\\n\"	; store a string somewhere with this value",
		"",
		".label start			; program start",
		"move 1 bill				; bill = 1",
		"move 2 charlie			; charlie = 2",
		"add bill charlie		; charlie = charlie+bill = 3",
		"",
		"comp bill charlie		; check if bill is <, >, =, or != to charlie",
		"move print microwave	; set our jump address to the instruction at print",
		"jumpne					; jump to microwave if bill != charlie",
		"stop					; stop is skipped by jumpne",
		"",
		".label print			; new section of code to jump to",
		"$PRINT msg				; macro for printing (see I/O)",
	].join('\n');

	return <div className="GonkASMEditor">
		<div className="GonkASMEditorTopbar">
			<button id="CompileButton" onClick={compile}>Compile</button>
			<button id="RunButton" onClick={compile}>Run</button>
			<button id="GuideButton" onClick={toggleGuide}>Guide</button>
		</div>
		<GonkASMEditorHeader />
		<div className="GonkASMEditorWindow" style={{ gridColumn: guide ? "span 1" : "span 2" }}>
			<Editor
				defaultLanguage="gonkASM"
				defaultValue={code}
				beforeMount={registerGonkASM}
				onMount={handleEditorMount}
				theme="gonkTheme"
				options={{
					minimap: {
						enabled: false
					}
				}}
			/>
		</div>
		{guide && <GonkASMGuide />}
	</div>;
};

export default GonkASMEditor;
