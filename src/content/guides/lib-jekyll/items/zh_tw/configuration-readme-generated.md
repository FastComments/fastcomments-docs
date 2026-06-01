設定來源有三處。後面的來源會勝出：

1. **全域預設** 在 `_config.yml` 底下的 `fastcomments:` 鍵。
2. **頁面上下文**，自動為頁面作用域的小工具派生（見下方）。
3. **標籤屬性** 直接寫在標籤本身。

因此，標籤上的 `url_id` 會覆蓋由頁面派生的值，而頁面派生的值又會覆蓋任何全域預設。

### 屬性語法

屬性為 `key=value` 的配對，使用 `snake_case`：

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Quoted** values (`"..."` or `'...'`) are literal strings.
- **Unquoted** `true`/`false` become booleans, and numbers become numbers.
- **Unquoted** anything else is resolved as a Liquid variable from the page context, e.g.
  `url_id=page.slug`. (Liquid does not expand `{% raw %}\{{ ... }}{% endraw %}` inside a tag's
  attributes, so use the bare `page.slug` form rather than `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case attribute and config keys are mapped automatically to the camelCase keys FastComments
expects (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, and so on). Any other option from the
[widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
passes straight through the same way.

### 由頁面派生的值

對於頁面作用域的 widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`)，這些值會自動從當前頁面填入，除非你自行設定：

- `url_id` ← `page.url` (a stable identifier independent of the visiting domain)
- `url` ← `site.url` + `page.url` (only when `url` is set in `_config.yml`)
- `page_title` ← `page.title`

Site-wide widgets (recent comments/discussions, top pages, reviews summary, user activity feed,
bulk count) are not tied to a page and do not derive these.

### 歐盟資料駐留

歐盟客戶請新增 `region: eu`，可以全域設定：

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

或是每個標籤設定： `{% raw %}{% fastcomments region="eu" %}{% endraw %}`。小工具將會從歐盟的 CDN 載入。