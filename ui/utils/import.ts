import fetch from "node-fetch";

const importUrl = "https://now-importer.now.sh/import";

export const importWebsite = async (url: string, token: string) => {
	const data = { debug: true, token, url };
	const response = await fetch(importUrl, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data)
	});
	const contentType = response.headers.get("content-type");

	if (!contentType || !contentType.includes("application/json")) {
		throw new Error(await response.text());
	}

	const json = await response.json();

	if (!response.ok) throw new Error(json.error);

	return json.url;
};
