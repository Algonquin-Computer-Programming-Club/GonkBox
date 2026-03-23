import { Monaco } from "@monaco-editor/react";
import { languages, } from "monaco-editor";

export const GonkASMCompletionItemProvider = (monaco: Monaco) => {
	return {
		provideCompletionItems: (model, position) => {
			var word = model.getWordUntilPosition(position);
			var range = {
				startLineNumber: position.lineNumber,
				endLineNumber: position.lineNumber,
				startColumn: word.startColumn,
				endColumn: word.endColumn,
			};
			var suggestions = [
				{
					label: "move",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "move ${1:source}, ${2:destination}",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range,

					documentation: "**move:** Copy the value of `source` to `dest`"
				},

				{
					label: "add",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "add ${1:source}, ${2:destination}",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range,

					documentation: "**add:** Add the value of `source` onto `dest`"
				},

				{
					label: "sub",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "sub ${1:source}, ${2:destination}",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range,

					documentation: "**sub:** Subtract the value of `source` from `dest`"
				},

				{
					label: "comp",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "comp ${1:arg1}, ${2:arg2}",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range,

					documentation: "**comp:** Compare `arg1` and `arg2`, store in `<cr>`"
				},

				{
					label: "jump",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "jump",
					range: range,
				},
				{
					label: "jumpe",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "jumpe",
					range: range,
				},
				{
					label: "jumpne",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "jumpne",
					range: range,
				},
				{
					label: "jumpl",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "jumpl",
					range: range,
				},
				{
					label: "jumpg",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "jumpg",
					range: range,
				},

				{
					label: "stop",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "stop",
					range: range,

					documentation: "**stop:** Stop execution of the program"
				},

				{
					label: "dbyte",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "dbyte",
					range: range
				},
				{
					label: "dbytes",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "dbytes ${1:n}",
					range: range
				},

				{
					label: "ibyte",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "ibyte ${1:def}",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range
				},
				{
					label: "ibytes",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "ibytes ${1:n} ${2:def}",
					range: range
				},

				{
					label: "istr",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "istr \"${1:str}\"",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range
				},
				{
					label: "istrn",
					kind: monaco.languages.CompletionItemKind.Keyword,
					// eslint-disable-next-line
					insertText: "istrn ${1:n} \"${2:str}\"",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule
							.InsertAsSnippet,
					range: range
				},
			];
			return { suggestions: suggestions };
		}
	} as languages.CompletionItemProvider;
}
