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
11. **Images must match the source exactly.** Reproduce every `<img>` tag, every markdown
    `![alt](url)` image, and every `[app-screenshot-start ... app-screenshot-end]` block
    exactly once, in source order. Never drop one, never duplicate one (and never emit the
    English text alongside the translation - that's how duplicates get introduced), and
    never alter an image path: `src="..."`, the markdown URL, and the `url=` / `selector=`
    / `clickSelector=` attributes are technical identifiers. Only `alt` / `title` text is
    translated. Inside `[app-screenshot-*]`, keep the `; ` separator between every
    attribute, including the one before `title=`.

Rule 11 is enforced by `trans validate-images`, a hard build gate - see below.

### app-screenshot blocks are rebuilt, not trusted

`[app-screenshot-*]` bodies are evaluated as **JavaScript** at build time, and the only
translatable fields in them are `title` and `alt`. So `trans run` does NOT keep the
translator's version of these blocks: `images::merge_screenshot_blocks` rebuilds each one
from the English source and copies over only the translated `title` / `alt`, escaping any
apostrophes. This is the same kind of deterministic post-processing as
`sanitize_inline_code_attrs`, and it exists because prompt rules did not work - in one CI
run the model dropped a word from the English sample text inside a percent-encoded `url=`
(11 locales), replaced a 2KB fixture URL with `'...'`, and left Hebrew apostrophes
unescaped in 17 files, all while the prompt explicitly forbade each one.

Note that no quoting convention avoids this: Hebrew uses both `'` (geresh, in `הווידג'ט`)
and `"` (gershayim, in `דוא"ל`), so single- and double-quoted values are equally exposed.
Escaping has to happen in code.

To repair the back-catalog (translations written before the merge existed):

```bash
./rust/target/release/trans validate-images --fix
```

It is idempotent, and `--fix` is a maintenance action only - `build.sh` runs the gate with
no arguments so a build never rewrites content.

### Validating translations

```bash
./rust/target/release/trans validate-images
```

Exits non-zero and lists every translated item that either (a) has different images than
the default-locale source, or (b) contains an `[app-screenshot-*]` block that no longer
parses as JavaScript. Case (b) matters because `sitegen` reacts to it by logging
`skip item ... error=eval app-screenshot config` and dropping the **entire page** - a
warning in a 6000-line build log that nobody sees.

It runs as its own `build.sh` phase after `trans check` / `trans run`, so a bad translation
fails the build rather than silently shipping a page with a missing, duplicated, or 404
image. `trans check` reports the same problems, and `trans run` re-translates any file that
has one (even when the translation cache says it's fresh) so the gate can't deadlock the
build. Extraction + comparison + merge live in `rust/trans/src/images.rs`; the walker is
`rust/trans/src/validate.rs`.

### Link targets are restored, not trusted

Same story, same shape, for URLs - `rust/trans/src/links.rs` plus
`./rust/target/release/trans validate-links [--fix]`. A translated page must link exactly
where the English page links, and the translator does not respect that on its own: it has
dropped `_status_` out of the middle of a path, injected a zero-width space into a hostname
(`fastcomment\u{200b}s-nim`), and truncated paths down to `docs/Models/`. Each one ships a
404 that only an external crawl ever notices.

So `trans run` runs `links::merge_link_urls` over fresh output, restoring the source's
targets positionally while keeping the translated link text, and `validate-links` catches
the back-catalog. Two things to know:

- **A link COUNT mismatch is not fatal.** There is no deterministic repair for a link the
  translator dropped or invented, so gating on it would wedge the build behind an LLM that
  may never produce the right count. Those files fail `validate::file_problem` instead,
  which makes `trans check` non-zero and `trans run` re-translate them. Only an *altered
  target* - same count, different URL - fails the build, and that class cannot survive
  `merge_link_urls`.
- **The label pattern allows one level of nested brackets.** Generated SDK pages are full of
  ``Returns: [`Option[T]`](url)``, and a pattern that stops at the first `]` matches none of
  them and reports a clean file. That is exactly how the zero-width space above survived the
  first pass at this check.

`validate::file_problem` is the single shared definition of "this translation is broken",
used by the gates, `check`, and `run`'s task discovery. Keep it that way: if `run` used a
narrower rule it would skip a file the gate rejects.

### Generated reference indexes are copied, not translated

`snapshot::source_is_reference_index` exempts a file from translation entirely when it has
50+ non-blank lines and under 5% of them are prose - an API endpoint table or a model list,
in other words. `run` copies the source verbatim for those, the same way it handles `en_us`.

This is not an optimization, it's a correctness fix.
`sdk-cpp/documentation-for-fastcomments-readme-generated.md` is 592 lines: 211 table rows,
360 `- [Type](url)` bullets, 8 headings, and **4 lines of real prose**. Translations of it
came back holding between 0 and 259 of the 360 links, with some URLs truncated to
`docs/Models/`. There is nothing to pair up, so no merge can repair it, so it fails link
parity forever - which means `run` re-translates all 66KB of it, per locale, on every build,
producing a fresh truncation each time. Shipping an English model index beats shipping a
truncated one.

Across the whole tree the predicate selects exactly five files (the cpp and nim endpoint
indexes, three sdk-php ones) with no near misses - the next-closest page is above 12% prose.
If you widen it, re-check that separation.

### Propagating a link-only English edit

`scripts/propagate-link-fixes.js` carries a URL-only change from `items/en/` into every
translated copy and reseals `src/translation-cache.json`, so a link fix across hundreds of
generated files doesn't queue thousands of DeepInfra calls. It refuses to touch anything it
isn't sure about: the English change must reduce to the same text with links stripped, each
rewrite must find the old URL as a whole link target the same number of times English had
it, and the result must end up with exactly English's URL list. Copies that fail any of
those keep their stale cache entry and re-translate normally.

**This is a maintenance action, never a build step.** As a standing `build.sh` phase it
would suppress re-translation on every English edit. Same rule as `validate-images --fix`.

### Batching strategy

For large translation runs, work in batches:
- Translate all files for one guide + one locale before moving to the next.
- Use `--guide` and `--locale` flags to scope the work.
- Commit after completing each guide or after a reasonable batch size.

## Generated links must resolve in the checkout

Every GitHub URL `sdkgen` emits is built from a path it has just stat'd in
`src/content/sdks-checkout/<sdk-id>/`, in all three places that build one:
`generators/readme.rs` (relative links in an upstream README), `generators/openapi.rs`
(`Returns:` links for go/java/php/python/ruby/swift/dart), and
`generators/ai/common.rs` (`Returns:` links for cpp/typescript/rust/nim). A path that
isn't there renders as unlinked text rather than a 404, and a directory gets `/tree/`
rather than `/blob/` (GitHub 301s the latter).

Two rules worth keeping:

- **Don't compute a filename you can find instead.** `openapi-generator`'s Ruby output puts
  `GetPageByURLIdAPIResponse` in `get_page_by_urlid_api_response.rb`, which no
  `underscore`-style algorithm predicts. `ruby_type_file_path` scans the models dir for
  `class <TypeName>`, the way `rust_type_file_path` / `go_type_file_path` already did.
- **Sort `read_dir` before taking the first match.** Directory order is unspecified (inode
  order on ext4), so an unsorted scan can emit a different URL on a different machine and
  flap 22 locales into re-translation. `sorted_file_names` exists for this.

`linkcheck.rs` is the backstop, run inside `guide::generate_one` on the markdown as it is
written: any link into the SDK's own repo that doesn't resolve becomes a validation error
and fails the build through the existing end-of-run report. It lives there rather than in a
later `build.sh` phase because that is the only point where the checkout is guaranteed
fresh - `checkout.rs` pulls at the start of the same run.

## Site generation

The static site is generated by the Rust `sitegen` crate (`rust/sitegen/src/build.rs`,
run via `./rust/target/release/sitegen build` in `build.sh`). `src/guides.js` is a
**legacy** reference generator — it is not run by the build. Templates live in
`src/templates/` (e.g. `page.html`). The deploy build also runs `trans check`/`trans run`
(the Rust `trans` tool, DeepInfra) to auto-fill missing translations before generating.

## SEO / canonical policy

**Each page is self-canonical per locale.** A translated page's `canonicalUrl` is its
OWN locale URL, NOT the English URL — set in `build.rs` as
`canonical_url = guide_link(&guide.id, locale, &locales.default_locale)`. This is the
correct multilingual setup: hreflang alternates and sitemap `<loc>` entries must point
to canonical URLs, so canonicalizing every locale to English is exactly what makes them
"non-canonical" (it caused ~3k `HREFLANG_POINTS_TO_NON_CANONICAL` + `SITEMAP_NON_CANONICAL`
in the Wrendex crawl). **Do NOT revert to canonicalize-to-English** — the old
"duplicate-content penalty" concern is unfounded when hreflang correctly links the
alternates (Google treats them as translations, not duplicates). `default_url` is still
used for the `x-default` hreflang and the stable comment `urlId` (shared across locales).
This matches the fastcomments blog's canonical policy.

## Social cards (og: / twitter:)

`page.html` and `index.html` emit the full Open Graph + Twitter card set
(`twitter:card` is `summary_large_image`, matching
`fastcomments-blog/src/templates/post.html`). `code.html` is `noindex` and is left alone.
`twitter:site` is deliberately absent: there is no X handle anywhere in fastcomments,
fastcomments-blog, or the live fastcomments.com HTML.

**Card images are generated per (guide, locale)** by `rust/sitegen/src/og_image.rs`, which
runs as a pre-pass in `build::run` before the page passes because every `page_ctx` needs
its card filename.

- 1200x630 (the 1.91:1 that `summary_large_image` wants). The old site-wide
  `https://fastcomments.com/images/og-card.png` is 1200x923, so X used to crop it.
- Rendered in headless Chrome from `src/templates/og-card.html` via `page.set_content()`.
  Chrome, not a Rust rasterizer, because ja/ko/zh/he need font fallback, bidi, and line
  wrapping. `set_content` leaves the document on `about:blank`, so the template must stay
  fully self-contained - the webfont and the guide icon are base64 data URIs and the logo
  is CSS-drawn. A relative URL there silently 404s.
- Output is `src/static/generated/images/og/<md5>.png`. That subdirectory survives
  `build.sh`'s `rm -f src/static/generated/*.*` (top-level files only), which is what makes
  the cache worth having.
- Filenames are content-addressed on `md5(title|kicker|icon|locale|CARD_VERSION)`, so the
  file existing IS the freshness check. **Bump `CARD_VERSION` after any design or template
  change**, otherwise every cached card stays stale forever. A cold build renders ~2500
  cards in ~3 minutes; every later build is a cache hit costing ~0.
- Failures are never fatal: no Chrome, a launch error, or `SITEGEN_OG_CARDS=0` all fall back
  to the old static `og-card.png` (declared at its real 1200x923).
- Only a full, unfiltered build prunes orphaned cards; a `--guide` / `--locale` run
  enumerates part of the site and would delete cards it isn't rebuilding.

## Meta descriptions

**`src/content/guides/<id>/meta-desc.txt` is the authored SEO meta description**, one per
guide, and it feeds `<meta name="description">`, `og:description`, and
`twitter:description`. Plain prose, one sentence or two, aim for 120-160 characters (Google
truncates around there). It is never rendered into the page body.

It is a separate file rather than a `meta.json` field for one reason: `trans` translates
files, not arbitrary JSON fields. `rust/trans/src/discover.rs` lists it in
`ROOT_LEVEL_SOURCES` alongside `intro.md` / `conclusion.md`, so `trans check` and
`trans run` pick it up with no extra pipeline, and translations land at
`items/<locale>/meta-desc.txt`. **To add a guide description, write the English file and
let the next build translate it** - do not hand-write the localized copies.

Note `ROOT_LEVEL_SOURCES` is also what `check.rs` tests to decide a guide is a pre-locale
flat structure needing migration. `authentication`, `sso`, and `wordpress` have no `items/`
directory at all, so before that check consulted the list they were skipped outright and
their descriptions would have shipped in English across all 22 translated locales.

**Resolution order** in `build_one_guide`, each tier localized where it can be:

1. `meta-desc.txt`, resolved `items/<locale>/` -> `items/<default>/` -> guide root
2. a `meta.json` `description`, but only when it differs from the default locale's -
   `trans` copies that field through untranslated, so an identical string is English text
   sitting on a translated page
3. an excerpt of the locale-resolved `intro.md`, via the shared `pipeline::html_to_text`
4. the template's title fallback

Two quirks of translated files that both the excerpt and `meta-desc.txt` reader work
around: every translated file comes back wrapped in `---` fences, which render as a
leading `<hr>` and would land verbatim in a `content="..."` attribute; and when the closing
fence sits directly under the last line of text, markdown reads it as a setext underline
and turns the whole paragraph into an `<h2>` (see `badges/items/ja_jp/intro.md`) - which
`html_to_text` then uppercases, because that is the right behavior for the search index it
was written for.
