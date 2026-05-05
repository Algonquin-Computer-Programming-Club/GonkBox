import { MarkerSeverity } from 'monaco-editor';
import { buildGonkASMProgram, ParseError, ProgramBinary, Tokenizer } from 'gonkbox-emu'
import { Monaco } from '@monaco-editor/react';

class GonkASMParser {
	program: ProgramBinary | null = null;

	parse(source: string, monaco: Monaco) {
		let tokenizer = new Tokenizer(source);
		try {
			let tokens = tokenizer.build();

			try {
				let program = buildGonkASMProgram(tokens);
				this.program = program;

				monaco.editor.removeAllMarkers("GonkASMParser");
			} catch (err) {
				if (err instanceof ParseError) {
					let token = err.get_token();
					console.error(err);
					if (token) {
						monaco.editor.setModelMarkers(monaco.editor.getModels()[0], "GonkASMParser", [{
							startLineNumber: token.get_line() + 1,
							endLineNumber: token.get_line() + 1,
							startColumn: token.get_range_start(),
							endColumn: token.get_range_end() + 1,
							message: err.get_description(),
							severity: MarkerSeverity.Error,
						}]);
					}
				} else {
					console.error(err);
				}
			}
		} catch (err) {
			console.log(err);
		}
	}

	getProgram(): ProgramBinary | null {
		return this.program;
	}
}

export default GonkASMParser;
