Config comes from three places. Later sources win:

1. **Global defaults** in `_config.yml` under the `fastcomments:` key.
2. **Page context**, derived automatically for page-scoped widgets (see below).
3. **Tag attributes** written on the tag itself.

So a `url_id` on the tag overrides the page-derived value, which overrides any global default.

### תחביר התכונות

Attributes are `key=value` pairs in `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Quoted** values (`"..."` or `'...'`) are literal strings.
- **Unquoted** `true`/`false` become booleans, and numbers become numbers.
- **Unquoted** anything else is resolved as a Liquid variable from the page context, e.g. `url_id=page.slug`. (Liquid does not expand `{% raw %}\{{ ... }}{% endraw %}` inside a tag's attributes, so use the bare `page.slug` form rather than `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case attribute and config keys are mapped automatically to the camelCase keys FastComments
expects (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, and so on). Any other option from the
[widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
passes straight through the same way.

### ערכים נגזרים מהדף

For the page-scoped widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) these are filled in automatically from the
current page unless you set them yourself:

- `url_id` ← `page.url` (a stable identifier independent of the visiting domain)
- `url` ← `site.url` + `page.url` (only when `url` is set in `_config.yml`)
- `page_title` ← `page.title`

Site-wide widgets (recent comments/discussions, top pages, reviews summary, user activity feed,
bulk count) are not tied to a page and do not derive these.

### שהות נתונים באיחוד האירופי

EU customers add `region: eu`, either globally:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

or per tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgets then load from the EU CDN.