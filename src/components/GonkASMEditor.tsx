import Editor, { Monaco } from '@monaco-editor/react';
import { editor } from 'monaco-editor';
import { useRef } from 'react';
import { GonkASMCompletionItemProvider } from '../language/GonkASMCompletionItemProvider';
import { GonkASMThemeData } from '../language/GonkASMThemeData';
import { GonkASMTokenProvider } from '../language/GonkASMTokenProvider';
import GonkASMEditorTopbar from './GonkASMEditorTopbar';

function GonkASMEditor() {
	const editorRef = useRef<editor.IStandaloneCodeEditor>(null);

	function registerGonkASM(monaco: Monaco) {
		monaco.languages.register({ id: "gonkASM" });

		monaco.languages.setMonarchTokensProvider("gonkASM", GonkASMTokenProvider);

		monaco.languages.registerCompletionItemProvider("gonkASM", GonkASMCompletionItemProvider(monaco));

		monaco.editor.defineTheme("gonkTheme", GonkASMThemeData);
	}

	function handleEditorMount(editor: editor.IStandaloneCodeEditor, _monaco: Monaco) {
		editorRef.current = editor;
	}

	const code = [
		"$INCLUDE \"io.gonk\" 		; someone else already wrote io for us; thats nice!",
		"",
		".label msg				; the strings name is msg when we're coding",
		"istr \"hello world!\\n\"	; store a string somewhere with this value",
		"",
		".label start			; program start",
		"move 1, %bill			; bill = 1",
		"move 2, %charlie		; charlie = 2",
		"add %bill, %charlie		; charlie = charlie+bill = 3",
		"",
		"comp %bill, %charlie	; check if bill is <, >, =, or != to charlie",
		"move print, %microwave	; set our jump address to the instruction at print",
		"jumpne					; jump to microwave if bill != charlie",
		"stop					; stop is skipped by jumpne",
		"",
		".label print			; new section of code to jump to",
		"$PRINT msg				; macro for printing (see I/O)",
	].join('\n');

	return <>
		<GonkASMEditorTopbar
			ref={editorRef}
		/>
		<Editor
			height="90vh"
			width="100vw"
			defaultLanguage="gonkASM"
			defaultValue={code}
			beforeMount={registerGonkASM}
			onMount={handleEditorMount}
			theme="gonkTheme"
		/>
	</>;
};

export default GonkASMEditor;
