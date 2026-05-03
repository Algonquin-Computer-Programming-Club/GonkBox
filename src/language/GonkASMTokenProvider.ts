import { languages } from "monaco-editor"

export const GonkASMTokenProvider: languages.IMonarchLanguage = {
	defaultToken: 'invalid',

	commands: [
		'dbyte', 'dbytes', 'ibyte', 'ibytes', 'istr', 'istrn',
		'dword', 'dwords', 'iword', 'iwords'
	],

	instructions: [
		'move',
		'add', 'sub', 'inc', 'dec', 'flip', 'comp',
		'jump', 'jumpe', 'jumpne', 'jumpl', 'jumpg',
		'stop'
	],

	registers: [
		'bill', 'charlie', 'tim',
		'bill_h', 'charlie_h', 'tim_h',
		'bill_l', 'charlie_l', 'tim_l',
		'b', 'c', 't',
		'b_h', 'c_h', 't_h',
		'b_l', 'c_l', 't_l',

		'microwave', 'm',

		'paul',

		'canada',
	],

	immediate: /[0-9]+/,
	immediateHex: /0[xX][0-9a-fA-F]+/,

	identifier: /[a-zA-Z_]+/,

	label: ['label'],

	macro: /\$([A-Z_]+)/,

	escapes: /\\(?:[nt\\"])/,

	ignoreCase: false,

	tokenizer: {
		root: [
			// identifiers and keywords
			[/@identifier/, {
				cases: {
					'@commands': 'command',
					'@instructions': 'instruction',
					'@registers': 'register',
					'@label': 'label',
					'@default': 'identifier'
				}
			}],

			// whitespace
			{ include: '@whitespace' },

			// numbers
			[/@immediateHex/, 'immediate.hex'],
			[/@immediate/, 'immediate'],

			// macros
			[/@macro/, 'macro'],

			// ram address bracket
			// eslint-disable-next-line
			[/\*/, 'rambracket'],

			// strings
			[/"([^"\\]|\\.)*$/, 'string.invalid'],
			[/"/, { token: 'string', bracket: '@open', next: '@string' }],
		],

		string: [
			[/[^\\"]+/, 'string'],
			[/@escapes/, 'string.escape'],
			[/\\./, 'string.escape.invalid'],
			[/"/, { token: 'string', bracket: '@close', next: '@pop' }]
		],

		whitespace: [
			[/[ \t\r\n]+/, 'white'],
			[/;.+/, 'comment']
		],
	},
}
