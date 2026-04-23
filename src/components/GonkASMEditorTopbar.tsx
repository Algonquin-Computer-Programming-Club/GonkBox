import { Monaco } from "@monaco-editor/react";
import { editor } from "monaco-editor";
import { RefObject, useRef } from "react";
import GonkASMParser from "../parser/GonkASMParser";

type GonkASMEditorTopbarProps = {
	editorRef: RefObject<editor.IStandaloneCodeEditor | null>
	monacoRef: RefObject<Monaco | null>
}

function GonkASMEditorTopbar({
	editorRef,
	monacoRef,
}: GonkASMEditorTopbarProps) {
	const parser = useRef<GonkASMParser>(null);
	function compile() {
		if (!parser.current)
			parser.current = new GonkASMParser();

		if (editorRef.current && monacoRef.current)
			parser.current.parse(editorRef.current.getValue(), monacoRef.current);
	}
	return <>
		<button onClick={compile}>Compile</button>
		<button onClick={compile}>Run</button>
	</>;
}

export default GonkASMEditorTopbar;
