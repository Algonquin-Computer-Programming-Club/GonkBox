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
	return <>
		<div className="GonkASMGuide">
			{isLoaded && <div dangerouslySetInnerHTML={{ __html: markdownHtml }}></div>}
		</div>
	</>;
}

export default GonkASMGuide;
