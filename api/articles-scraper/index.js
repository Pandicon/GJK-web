// TODO: When there are multiple pages in a gallery, only the first one gets scraped (see posts page 262, post id post-6298 for an example)

const puppeteer = require('puppeteer');
const cheerio = require('cheerio');
const fs = require('fs');
const moment = require('moment-timezone');
const TurndownService = require('turndown');

const max_pages = 1;
const start_page = 263;
const last_page = start_page + max_pages - 1;

const delay_between_articles_ms = 200;

const classes_to_ignore = [
	'sharedaddy',
	'sd-sharing-enabled',
	'sd-content',
	'sd-block',
	'share-facebook',
	'share-twitter',
	'share-google-plus-1'
];
const phrases_to_ignore_all = [
	'Click to share on Facebook (Otevře se v novém okně)',
	'Sdílet na Twitteru',
	'Sdílet na Google+'
];

function delay(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
	const turndownService = new TurndownService();
	let pageGJK = start_page;
	let baseUrl = `https://gjk.cz/page/`;
	let url = `${baseUrl}${pageGJK}`;
	const browser = await puppeteer.launch();
	const page = await browser.newPage();
	let found = false;
	let all_posts = [];
	while (!found && pageGJK <= last_page) {
		console.log(
			`${pageGJK - start_page + 1}/${
				last_page - start_page + 1
			} (page ${pageGJK})`
		);
		url = `${baseUrl}${pageGJK}`;
		await page.goto(url);

		const pageData = await page.evaluate(() => {
			return {
				html: document.documentElement.innerHTML,
				width: document.documentElement.clientWidth,
				height: document.documentElement.clientHeight
			};
		});

		const $ = cheerio.load(pageData.html);
		const posts_page = $('article');

		for (const element of posts_page) {
			let post = {};
			let postId = element.attribs.id;
			let postTitleObject = $(element).children('.post-title');
			let postLinkObject = $(postTitleObject).children('a');
			let postTitle = postLinkObject.text();
			let postLink = postLinkObject.attr('href');
			let postmetaObject = $(element).children('.postmeta');
			let postmetaDateObject = $(postmetaObject)
				.children('.meta-date')
				.children('a');
			let postmetaAuthorObject = $(postmetaObject)
				.children('.meta-author')
				.children('a');
			let postmetaCategoriesObject = $(postmetaObject)
				.children('.meta-category')
				.children('a');
			let postTime = postmetaDateObject.attr('title');
			let postDate = postmetaDateObject.children('time').text();
			let post_thumbnail = $(element).find('img.wp-post-image');
			if (post_thumbnail !== null) {
				let src = post_thumbnail.attr('src');
				post['thumbnail_src'] = src ?? null;
			}
			let post_unix_timestamp = moment
				.tz(
					postDate + ' ' + postTime,
					'DD.MM.YYYY HH:mm',
					'Europe/Prague'
				)
				.unix();
			let postAuthor = '';
			let postCategories = '';
			postmetaAuthorObject.each(function (index, element) {
				if (index != 0) postAuthor += ', ';
				postAuthor += element.children[0].data;
			});
			postmetaCategoriesObject.each(function (index, element) {
				if (index != 0) postCategories += ', ';
				postCategories += element.children[0].data;
			});

			const details = async (postLink) => {
				try {
					await page.goto(postLink, { waitUntil: 'networkidle2' });
				} catch (error) {
					console.error('Navigation error:', error);
				}
				await page.waitForSelector('article');

				const pageData = await page.evaluate(() => {
					return {
						html: document.documentElement.innerHTML,
						width: document.documentElement.clientWidth,
						height: document.documentElement.clientHeight
					};
				});

				const $ = cheerio.load(pageData.html);
				const post_text_div = $('div.entry.clearfix');
				const topLevelAll = post_text_div.find('div > *');
				const topLevelPs = post_text_div.find('div > p');

				const all_elements = [];
				const paragraphs_html = [];
				topLevelAll.each((i, el) => {
					const element = $(el);
					if (
						!classes_to_ignore.some((cl) => element.hasClass(cl)) &&
						classes_to_ignore.every(
							(cl) => element.find(`li.${cl}`).length === 0
						)
					) {
						all_elements.push(element);
						paragraphs_html.push(element.html());
					}
				});

				/*topLevelPs.each((i, el) => {
					const element = $(el);
					if (
						!classes_to_ignore.some((cl) => element.hasClass(cl)) &&
						classes_to_ignore.every(
							(cl) => element.find(`li.${cl}`).length === 0
						)
					) {
						all_elements.push(element);
						paragraphs_html.push(element.html());
					}
				});*/

				const paragraphs_markdown = paragraphs_html
					.map((html) => turndownService.turndown(html))
					.filter(
						(chunk) =>
							!phrases_to_ignore_all.every((ph) =>
								chunk.includes(ph)
							) && chunk != ''
					);
				// console.log(paragraphs_markdown);
				return paragraphs_markdown;
			};

			await delay(delay_between_articles_ms);
			let article_text = await details(postLink);

			post['id'] = postId;
			post['title'] = postTitle;
			post['link'] = postLink;
			post['date'] = postDate;
			post['time'] = postTime;
			post['unix'] = post_unix_timestamp;
			post['author'] = postAuthor;
			post['categories'] = postCategories;
			post['contents'] = article_text;

			all_posts.push(post);
		}
		pageGJK += 1;
	}

	all_posts.reverse();
	fs.writeFile(
		`posts--pages-${start_page}-to-${last_page}.txt`,
		JSON.stringify(all_posts, null, 2),
		'utf8',
		(err) => {
			if (err) throw err;
			console.log('The posts have been saved!');
		}
	);
}

main();
