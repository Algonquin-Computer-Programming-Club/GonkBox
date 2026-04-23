import { Monaco } from '@monaco-editor/react';
import { Token } from 'monaco-editor';
import { GonkASMToken, GonkASMTokenType, buildGonkASMProgram } from 'gonkbox-emu'

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
	parse(source: string, monaco: Monaco) {
		var lines = monaco.editor.tokenize(source, "gonkASM");
		var sourceLines = source.split("\n");

		var preparedTokens: GonkASMToken[] = [];
		for (let i = 0; i < sourceLines.length; i++) {
			let tokens = lines[i];
			for (let j: number = 0; j < tokens.length; j++) {
				this.processMonarchToken(sourceLines[i],
					tokens[j], j !== tokens.length - 1 ? tokens[j + 1] : null,
					preparedTokens);
			}
		}

		buildGonkASMProgram(preparedTokens);
	}

	processMonarchToken(source: string, t: Token, next: Token | null, preparedTokens: GonkASMToken[]) {
		let value: string = source.substring(t.offset, next ? next.offset : source.length).trim();
		let type: GonkASMTokenType = tokenTypeMap[t.type];
		console.log(t.type, type);
		if (type === undefined) return;

		preparedTokens.push(new GonkASMToken(value, type));
	}
}

export default GonkASMParser;
