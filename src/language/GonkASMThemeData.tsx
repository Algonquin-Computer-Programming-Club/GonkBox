import * as monaco from "monaco-editor/esm/vs/editor/editor.api";

export const GonkASMThemeData: monaco.editor.IStandaloneThemeData = {
	base: "vs",
	inherit: false,
	rules: [
		{ token: "command", foreground: "458588" },
		{ token: "instruction", foreground: "d65d0e" },

		{ token: "identifier", foreground: "fbf1c7" },
		{ token: "register", foreground: "83a598" },

		{ token: "label", foreground: "b16286" },

		{ token: "immediate", foreground: "fe8019" },

		{ token: "macro", foreground: "b8bb26" },

		{ token: "comment", foreground: "689d6a" },

		{ token: "rambracket", foreground: "689d6a" },

		{ token: "string", foreground: "8ec07c" },
		{ token: "string.escape", foreground: "fb4934" },
		{ token: "string.invalid", foreground: "689d6a", fontStyle: "bold" },
		{ token: "string.escape.invalid", foreground: "cc241d", fontStyle: "bold" },

		{ token: "character", foreground: "b16286" },
	],
	colors: {
		"editor.foreground": "#fbf1c7",
		"editor.background": "#1d2021",

		"editorCursor.foreground": "#fbf1c7",

		"editor.lineHighlightBorder": "#282828",

		"editorLineNumber.activeForeground": "#fbf1c7",
		"editorLineNumber.foreground": "#a89984",
	},
};
