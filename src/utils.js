const fs = require('fs');
const handlebars = require('handlebars');

function getCompiledTemplate(templatePath, data) {
    return handlebars.compile(fs.readFileSync(templatePath, 'utf8'))(data);
}

module.exports = { getCompiledTemplate };
