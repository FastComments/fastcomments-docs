process.env.OPENAI_API_KEY = 'dummy';
const path = require('path');
const fs = require('fs');
const root = process.cwd();
const tw = fs.readFileSync(path.join(root, 'src/translate-with-gpt.js'), 'utf8');
const patched = tw
    .replace(/if \(require\.main === module\)[\s\S]*$/, '')
    .replace(/require\('\.\//g, `require('${root}/src/`)
    + '\nmodule.exports = TranslationClient;\n';
fs.writeFileSync('/tmp/tw-patched.js', patched);
const TranslationClient = require('/tmp/tw-patched.js');
const c = new TranslationClient();

const fixDir = path.join(root, 'rust/trans/tests/fixtures');
const fixtures = [
    {name: 'hello', content: 'Hello world.'},
    {name: 'markdown_with_inline_code', content: fs.readFileSync(path.join(fixDir, 'markdown_with_inline_code.md'), 'utf8')},
    {name: 'apostrophe_in_attrs',       content: fs.readFileSync(path.join(fixDir, 'apostrophe_in_attrs.md'),       'utf8')},
    {name: 'fenced_code',                content: fs.readFileSync(path.join(fixDir, 'fenced_code.md'),                'utf8')},
    {name: 'api_resource_header',        content: fs.readFileSync(path.join(fixDir, 'api_resource_header.md'),        'utf8')},
];
const out = {};
for (const loc of ['fr_fr', 'de_de', 'ja_jp']) {
    out[loc] = {system: c.getSystemMessage(loc), prompts: {}};
    for (const f of fixtures) {
        out[loc].prompts[f.name] = c.buildPrompt(f.content, loc);
    }
}
process.stdout.write(JSON.stringify(out, null, 2) + '\n');
