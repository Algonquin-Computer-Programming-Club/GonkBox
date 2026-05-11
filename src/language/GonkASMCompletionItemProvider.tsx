import { Monaco } from "@monaco-editor/react";
import { languages, } from "monaco-editor";

export const GonkASMCompletionItemProvider = (_monaco: Monaco) => {
	return {
		provideCompletionItems: (model, position) => {
			var word = model.getWordUntilPosition(position);
			var range = {
				startLineNumber: position.lineNumber,
				endLineNumber: position.lineNumber,
				startColumn: word.startColumn,
				endColumn: word.endColumn,
			};
			var registers = [
				"bill", "bill_l", "bill_h",
				"charlie", "charlie_l", "charlie_h",
				"tim", "tim_l", "tim_h",
				"microwave"
			];
			var instructions = [
				{ name: "move", args: 2 },

				{ name: "add", args: 2 },
				{ name: "sub", args: 2 },
				{ name: "mul", args: 2 },
				{ name: "div", args: 2 },
				{ name: "inc", args: 1 },
				{ name: "dec", args: 1 },

				{ name: "comp", args: 2 },
				{ name: "jump", args: 0 },
				{ name: "jumpe", args: 0 },
				{ name: "jumpne", args: 0 },
				{ name: "jumpg", args: 0 },
				{ name: "jumpl", args: 0 },

				{ name: "dlogn", args: 1 },
				{ name: "dlogc", args: 1 },
				{ name: "dlogs", args: 1 },
			];
			var commands = [
				{ name: "dbyte", args: 0 },
				{ name: "dbytes", args: 1 },
				{ name: "ibyte", args: 1 },
				{ name: "ibytes", args: 2 },

				{ name: "istr", args: 1 },
				{ name: "istrn", args: 2 },

				{ name: "dword", args: 0 },
				{ name: "dwords", args: 1 },
				{ name: "iword", args: 1 },
				{ name: "iwords", args: 2 },
			];
			var suggestions: languages.CompletionItem[] = [];
			registers.forEach((x) => {
				suggestions.push({
					label: x,
					kind: languages.CompletionItemKind.Keyword,
					insertText: x,
					range,
				} as languages.CompletionItem);
			});

			instructions.forEach((x) => {
				let insert = x.name;
				for (let i = 0; i < x.args; i++) {
					insert += " ${" + (i + 1) + ":arg" + (i + 1) + "}";
				}
				suggestions.push({
					label: x.name,
					kind: languages.CompletionItemKind.Function,
					insertText: insert,
					insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
					range,
				} as languages.CompletionItem);
			});

			commands.forEach((x) => {
				let insert = x.name;
				for (let i = 0; i < x.args; i++) {
					insert += " ${" + (i + 1) + ":arg" + (i + 1) + "}";
				}
				suggestions.push({
					label: x.name,
					kind: languages.CompletionItemKind.Constructor,
					insertText: insert,
					insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
					range,
				} as languages.CompletionItem);
			});
			return { suggestions: suggestions };
		}
	} as languages.CompletionItemProvider;
}
