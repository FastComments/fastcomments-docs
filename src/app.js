const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const marked = require('marked');
const handlebars = require('handlebars');
const byteSize = require('byte-size');
const prettyBytes = require('pretty-bytes');
const readingTime = require('reading-time');

const YEAR_STR = '2020';
const CONTENT_DIR = path.join(__dirname, 'content');
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const STATIC_DIR = path.join(__dirname, 'static');

var posts = [];
fs.readdirSync(CONTENT_DIR).forEach(function(item) {
	const title = item.replace('\.md', '');
	const urlId = title.toLowerCase().replace(/ /g, '-') + '.html';
	const fullUrl = 'https://blog.winricklabs.com/' + urlId;

	let fileContent = fs.readFileSync(path.join(CONTENT_DIR, item), 'utf8');
	fileContent = fileContent.replace('[postlink]', `<a href="${fullUrl}">`);
	fileContent = fileContent.replace('[/postlink]', `</a>`);	let html = marked(fileContent);
	const postByteSize = byteSize(html.length).toString();

	const imageSrcs = html.match(/data-src\s*=\s*"?(.+?)["|\s]/g);
	let mediaSize = 0;
	if(imageSrcs) {
		for(const imageSrc of imageSrcs) {
			mediaSize += fs.statSync(path.join(STATIC_DIR, imageSrc.substring(10, imageSrc.length - 1))).size; // Would be nice to fix regex to not include src=""
		}
	}
	mediaSize = prettyBytes(mediaSize);
	const fileSizeString = imageSrcs ? `${postByteSize} - ${mediaSize}` : postByteSize;
	html = html.replace('[filesize]', fileSizeString);
	const readTime = readingTime(html).text;
	html = html.replace('[readtime]', readTime);

	const dateString = title.substring(1, title.indexOf(')'));
	const ctime = new Date(dateString);
	const dateStringReadable = ctime.toDateString();

	html = html.replace('[postdate]', dateStringReadable);

	let titleNoDate = title.substring(title.indexOf(')') + 1, title.length);
	if (titleNoDate.startsWith(' - ')) {
		titleNoDate = titleNoDate.substring(3, titleNoDate.length);
	}

	posts.push({
		html: html,
		title: title,
		titleNoDate: titleNoDate,
		urlId: urlId,
		fullUrl: fullUrl,
		dateTimeObj: ctime,
		dateTime: ctime.getTime(),
		dateStringReadable: dateStringReadable,
		minSize: postByteSize,
		mediaSize: mediaSize,
		fileSizeString: fileSizeString,
		hasMedia: !!imageSrcs,
		readTime: readTime
	});
});

posts.sort(function(a, b) {
	if(a.dateTime === b.dateTime) {
		return 0;
	}
	return a.dateTime > b.dateTime ? -1 : 1;
});

let footerYears = YEAR_STR;
if(new Date().getFullYear() > YEAR_STR) {
	footerYears += ' - ' + new Date().getFullYear();
}

fs.readdirSync(TEMPLATE_DIR).forEach(function(item) {
	if(item === 'index.html') {
		fs.writeFileSync(path.join(STATIC_GENERATED_DIR, item), getCompiledTemplate(item, {
			posts: posts,
				footerYears: footerYears
		}), 'utf8');
	}
	else if (item === 'post.html') {
		posts.forEach(function(post) {
			const html = getCompiledTemplate(item, {
				post: post,
				posts: posts,
				footerYears: footerYears
			});
			fs.writeFileSync(path.join(STATIC_GENERATED_DIR, post.urlId), html, 'utf8');
		});
	}
});

function getCompiledTemplate(templateName, data) {
	return handlebars.compile(fs.readFileSync(path.join(TEMPLATE_DIR, templateName), 'utf8'))(data);
}

console.log(`Execution Time: ${Date.now() - startTime}ms`);
