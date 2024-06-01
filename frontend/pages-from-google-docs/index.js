const axios = require('axios');
const fs = require('fs');
const stream = require('stream');
const util = require('util');

const { pages_from_google_docs: config } = require('../config.json');

const pipeline = util.promisify(stream.pipeline);

async function download_google_doc_as_html(config) {
	const url = `https://docs.google.com/document/d/${config.document_id}/export?format=html`;

	const response = await axios.get(url, { responseType: 'stream' });
	const file_path = `./pages/${config.file_name}.html`;
	await pipeline(response.data, fs.createWriteStream(file_path));

	console.log(`File downloaded at ${file_path}`);
}

function download_loop() {
	console.log('Downloading static pages from google docs');
	for (const page of config.pages) {
		console.log(`Downloading page '${page.file_name}'`);
		download_google_doc_as_html(page).catch(console.error);
	}
}

download_loop();

setInterval(() => {
	download_loop();
}, config.interval);
