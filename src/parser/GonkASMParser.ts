import { MarkerSeverity, Token } from 'monaco-editor';
import { GonkASMToken, GonkASMTokenType, buildGonkASMProgram, ParseError, ProgramBinary } from 'gonkbox-emu'
import { Monaco } from '@monaco-editor/react';

const tokenTypeMap: Record<string, GonkASMTokenType> = {
	"command.gonkASM": GonkASMTokenType.Command,
	"instruction.gonkASM": GonkASMTokenType.Instruction,

	"rambracket.gonkASM": GonkASMTokenType.RamBracket,

	"label.gonkASM": GonkASMTokenType.Label,

	"register.gonkASM": GonkASMTokenType.Register,
	"identifier.gonkASM": GonkASMTokenType.Identifier,

	"immediate.gonkASM": GonkASMTokenType.ImmediateLiteral,

	"string.gonkASM": GonkASMTokenType.StringLiteral,
	"string.escape.gonkASM": GonkASMTokenType.StringLiteralEscape,

	"macro.gonkASM": GonkASMTokenType.Macro,
};

class GonkASMParser {
	program: ProgramBinary | null = null;

	parse(source: string, monaco: Monaco) {
		var lines = monaco.editor.tokenize(source, "gonkASM");
		var sourceLines = source.split("\n");

		var preparedTokens: GonkASMToken[] = [];
		for (let i = 0; i < sourceLines.length; i++) {
			let tokens = lines[i];
			for (let j: number = 0; j < tokens.length; j++) {
				this.processMonarchToken(sourceLines[i], i,
					tokens[j], j !== tokens.length - 1 ? tokens[j + 1] : null,
					preparedTokens);
			}
		}

		try {
			let program = buildGonkASMProgram(preparedTokens);
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
			}
		}
	}

	processMonarchToken(source: string, line: number, t: Token, next: Token | null, preparedTokens: GonkASMToken[]) {
		let value: string = source.substring(t.offset, next ? next.offset : source.length).trim();
		let type: GonkASMTokenType = tokenTypeMap[t.type];
		if (type === undefined) return;

		preparedTokens.push(new GonkASMToken(value, type, line, t.offset, value.length));
	}

	getProgram(): ProgramBinary | null {
		return this.program;
	}
}

export default GonkASMParser;
