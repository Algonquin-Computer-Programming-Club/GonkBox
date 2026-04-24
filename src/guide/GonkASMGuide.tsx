import { marked } from "marked";
import { useEffect, useState } from "react";

function GonkASMGuide() {
	const [markdownHtml, setMarkdownHtml] = useState("");
	const [isLoaded, setIsLoaded] = useState(false);

	useEffect(() => {
		fetch("GonkASM.md", {
			headers: {
				'Content-Type': 'application/text',
				'Accept': 'application/text'
			}
		})
			.then((res) => res.text())
			.then((text) => marked.parse(text))
			.then((html) => {
				setMarkdownHtml(html);
				setIsLoaded(true);
			});
	});
	if (!isLoaded) {
		return (
			<p>Guide not loaded, please wait.</p>
		);
	}
	return (
		<div className="GonkASMGuide">
			<div dangerouslySetInnerHTML={{ __html: markdownHtml }}></div>
		</div >
	);
}

export default GonkASMGuide;
