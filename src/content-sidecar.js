/**
 * Build-time HTTP sidecar consumed by the Rust indexer and sitegen.
 *
 * Only owns one piece of behavior: highlight.js syntax highlighting,
 * which we keep in Node to preserve byte-exact compatibility with the
 * existing rendered code blocks. The marker config evaluator
 * (vm.runInContext) was previously here but moved in-process to
 * Rust via QuickJS — see rust/shared/src/markers/qjs.rs.
 *
 *   POST /highlight  -> wraps highlight.js (matches src/guides.js:26-33).
 *   GET  /health     -> liveness check used by Rust supervisors.
 *
 * The Rust indexer/sitegen auto-starts this process, reads the port from
 * `.content-sidecar-port`, then issues HTTP requests during builds. The
 * production search server never talks to this sidecar.
 *
 * Run manually:
 *   CONTENT_SIDECAR_PORT=4500 node src/content-sidecar.js
 */

const express = require('express');
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

// /eval-marker was removed once the Rust pipeline embedded QuickJS
// (rust/shared/src/markers/qjs.rs). The Node sidecar now only carries
// the highlight.js binding.

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
