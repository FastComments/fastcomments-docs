/**
 * Build-time HTTP sidecar consumed by the Rust indexer.
 *
 * Owns the two pieces of behavior we cannot run from Rust without embedding a
 * JS runtime:
 *
 *   POST /highlight    -> wraps highlight.js (matches src/guides.js:26-33).
 *   POST /eval-marker  -> wraps vm.runInContext for inline-code,
 *                         code-example, and api-resource-header marker
 *                         configs (see src/inline-code-generator.js,
 *                         src/code-example-generator.js,
 *                         src/api-resource-header-generator.js).
 *
 * The Rust indexer auto-starts this process, reads the port from
 * `.content-sidecar-port`, then issues HTTP requests during indexing. The
 * production search server never talks to this sidecar.
 *
 * Run manually:
 *   CONTENT_SIDECAR_PORT=4500 node src/content-sidecar.js
 */

const express = require('express');
const vm = require('vm');
const hljs = require('highlight.js');
const fs = require('fs');
const path = require('path');

const app = express();
app.use(express.json({ limit: '10mb' }));

app.get('/health', (req, res) => {
    res.send({ ok: true });
});

app.post('/highlight', (req, res) => {
    const body = req.body || {};
    const code = body.code;
    const lang = body.lang;
    if (typeof code !== 'string') {
        return res.status(400).send({ error: 'code is required (string)' });
    }
    try {
        let result;
        if (lang && hljs.getLanguage(lang)) {
            result = hljs.highlight(code.trim(), { language: lang });
        } else {
            result = hljs.highlightAuto(code.trim());
        }
        res.send({ html: result.value, language: result.language || null });
    } catch (e) {
        res.status(500).send({ error: e.message });
    }
});

app.post('/eval-marker', (req, res) => {
    const body = req.body || {};
    const kind = body.kind;
    const configSource = body.config_source;
    if (typeof configSource !== 'string') {
        return res.status(400).send({ error: 'config_source is required (string)' });
    }
    if (
        kind !== 'inline-code' &&
        kind !== 'code-example' &&
        kind !== 'api-resource-header' &&
        kind !== 'related-parameter'
    ) {
        return res.status(400).send({ error: `unknown marker kind: ${kind}` });
    }

    const args = {};
    vm.createContext(args);
    try {
        if (kind === 'inline-code') {
            args.globals = {};
        }
        vm.runInContext(configSource, args);
    } catch (e) {
        return res.status(400).send({
            error: `eval failed: ${e.message}`,
            input: configSource,
        });
    }
    if (kind === 'inline-code') {
        delete args.globals;
    }

    let plain;
    try {
        plain = JSON.parse(JSON.stringify(args));
    } catch (e) {
        return res.status(500).send({
            error: `marker result not json-serializable: ${e.message}`,
        });
    }
    res.send(plain);
});

const requestedPort = parseInt(process.env.CONTENT_SIDECAR_PORT || '0', 10);
const portFile = process.env.CONTENT_SIDECAR_PORT_FILE
    || path.join(__dirname, '..', '.content-sidecar-port');

const server = app.listen(requestedPort, '127.0.0.1', () => {
    const port = server.address().port;
    fs.writeFileSync(portFile, String(port), 'utf8');
    console.log(`content-sidecar listening on 127.0.0.1:${port}`);
});

function shutdown() {
    server.close(() => {
        try { fs.unlinkSync(portFile); } catch (_) {}
        process.exit(0);
    });
    setTimeout(() => process.exit(0), 2000).unref();
}
process.on('SIGTERM', shutdown);
process.on('SIGINT', shutdown);
