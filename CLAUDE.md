# FastComments Docs

## Translating Documentation

This repo has 28 target locales. Use `src/claude-translate.js` to manage translations — you ARE the translator (no external API needed).

### CLI Tool

```bash
# Find what needs translating
node src/claude-translate.js list [--locale fr_fr] [--guide api] [--limit 20]
node src/claude-translate.js list-meta [--locale fr_fr]
node src/claude-translate.js list-ui [--locale fr_fr]

# Read the English source
node src/claude-translate.js source <guideId> <filename>

# Save a translation (writes file + updates translation-cache.json)
node src/claude-translate.js save <guideId> <locale> <filename> /tmp/translated.md
node src/claude-translate.js save-meta <guideId> <locale> /tmp/translated-meta.json
```

### Workflow for guide files

1. Run `node src/claude-translate.js list` to find missing/stale translations.
2. Read the source with `node src/claude-translate.js source <guideId> <file>`.
3. Translate the content following the rules below.
4. Write the translation to a temp file, then `node src/claude-translate.js save <guideId> <locale> <file> /tmp/translated.md`.
5. Repeat. Process one locale at a time for a given file — translations within the same locale share context.

### Workflow for meta.json files

1. Run `node src/claude-translate.js list-meta` to find what's needed.
2. Read the source: `cat src/content/guides/<guideId>/meta.json`.
3. Translate the `name`, `pageHeader`, and each `itemsOrdered[].name` and `itemsOrdered[].subCat` value. Do NOT translate `file`, `icon`, or `type` fields.
4. Write the translated JSON to a temp file, then `node src/claude-translate.js save-meta <guideId> <locale> /tmp/meta.json`.

### Translation rules

When translating documentation files:

1. **DO NOT translate** code inside `[inline-code-start]...[inline-code-end]` blocks — preserve exactly as-is.
2. **DO translate** the `title` attribute inside `[inline-code-attrs-start ... inline-code-attrs-end]` tags. Keep all other attributes (`type`, `isFunctional`, `useDemoTenant`) exactly as-is.
3. When translating a title that contains an apostrophe inside a single-quoted value (e.g. French `d'utilisation`), escape it: `title = 'Exemple d\'utilisation'`.
4. **DO NOT translate** URLs, API endpoints, variable names, CSS class names, HTML tag names, or technical identifiers.
5. **DO NOT translate** property names in TypeScript/JavaScript interfaces or code examples.
6. **Preserve** all markdown formatting: headers (`##`), lists (`-`, `1.`), bold (`**`), links (`[text](url)`), etc.
7. **Preserve** all special tags exactly: `[inline-code-*]`, `[api-resource-header-*]`, `[code-example-*]`.
8. **Preserve** inline code (backtick spans like \`tenantId\`) — do not translate the content.
9. Keep the same line structure and paragraph breaks as the source.
10. For `en_us` locale, just copy the source content verbatim (no translation needed).

### Batching strategy

For large translation runs, work in batches:
- Translate all files for one guide + one locale before moving to the next.
- Use `--guide` and `--locale` flags to scope the work.
- Commit after completing each guide or after a reasonable batch size.
