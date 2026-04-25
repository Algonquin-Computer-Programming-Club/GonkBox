import { languages } from "monaco-editor"

export const GonkASMTokenProvider: languages.IMonarchLanguage = {
	defaultToken: 'invalid',

	commands: [
		'dbyte', 'dbytes', 'ibyte', 'ibytes', 'istr', 'istrn',
		'dword', 'dwords', 'iword', 'iwords'
	],

	instructions: [
		'move', 'moveb',
		'add', 'sub', 'inc', 'dec', 'flip', 'comp',
		'jump', 'jumpe', 'jumpne', 'jumpl', 'jumpg',
		'stop'
	],

	separator: /,/,

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

	label: /\.label/,

	macro: /\$([A-Z_]+)/,

	escapes: /\\(?:[nt\\"])/,

	ignoreCase: true,

	tokenizer: {
		root: [
			// identifiers and keywords
			[/@identifier/, {
				cases: {
					'@commands': 'command',
					'@instructions': 'instruction',
					'@registers': 'register',
					'@default': 'identifier'
				}
			}],

			// whitespace
			{ include: '@whitespace' },

			// labels
			[/(^@label)(\s+)(@identifier)/, ['label', 'white', 'identifier']],

			// argument separator
			[/@separator/, 'separator'],

			// numbers
			[/@immediateHex/, 'immediate.hex'],
			[/@immediate/, 'immediate'],

			// macros
			[/@macro/, 'macro'],

			// ram address brackets
			// eslint-disable-next-line
			[/[\[\]]/, 'rambracket'],

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
