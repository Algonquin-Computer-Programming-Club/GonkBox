import { editor } from "monaco-editor";
import { RefObject } from "react";

type GonkASMEditorTopbarProps = {
	ref: RefObject<editor.IStandaloneCodeEditor | null>
}

function GonkASMEditorTopbar({
	ref,
}: GonkASMEditorTopbarProps) {
	function compile() {
		alert(ref?.current?.getValue());
	}
	return <>
		<button onClick={compile}>Compile</button>
		<button onClick={compile}>Run</button>
	</>;
}

export default GonkASMEditorTopbar;
