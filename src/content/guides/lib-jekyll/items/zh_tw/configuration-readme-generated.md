設定來自三個地方，後面的來源會覆蓋前面的：

1. `_config.yml` 中 `fastcomments:` 鍵下的**全域預設值**。
2. **頁面上下文**，針對頁面範圍的小工具自動推導（見下方）。
3. 寫在標籤本身的**標籤屬性**。

因此標籤上的 `url_id` 會覆蓋頁面推導的值，而後者又會覆蓋任何全域預設值。

### 屬性語法

屬性是以 `snake_case` 表示的 `key=value` 配對：

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **加引號**的值（`"..."` 或 `'...'`）為字面字串。
- **未加引號**的 `true`/`false` 會轉為布林值，數字則轉為數字。
- **未加引號**的其他任何內容會從頁面上下文中解析為 Liquid 變數，例如
  `url_id=page.slug`。（Liquid 不會在標籤的屬性中展開 `{% raw %}\{{ ... }}{% endraw %}`，
  因此請使用裸的 `page.slug` 形式，而不是 `"{% raw %}\{{ page.slug }}{% endraw %}"`。）

snake_case 的屬性與設定鍵會自動對應到 FastComments 預期的 camelCase 鍵
（`tenant_id` → `tenantId`、`url_id` → `urlId`、`page_title` → `pageTitle`、
`has_dark_background` → `hasDarkBackground`，依此類推）。
[小工具設定](https://docs.fastcomments.com/guide-customizations-and-configuration.html)中的任何其他選項
也會以相同方式直接傳遞。

### 頁面推導的值

對於頁面範圍的小工具（`fastcomments`、`fastcomments_comment_count`、`fastcomments_live_chat`、
`fastcomments_collab_chat`、`fastcomments_image_chat`），除非您自行設定，否則以下值會從
目前頁面自動填入：

- `url_id` ← `page.url`（與造訪網域無關的穩定識別碼）
- `url` ← `site.url` + `page.url`（僅當 `_config.yml` 中設定了 `url` 時）
- `page_title` ← `page.title`

全站小工具（最新留言/討論、熱門頁面、評論摘要、使用者活動動態、
批次計數）不綁定於特定頁面，因此不會推導這些值。

### EU 資料駐留

EU 客戶可新增 `region: eu`，可以全域設定：

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

或在每個標籤上設定：`{% raw %}{% fastcomments region="eu" %}{% endraw %}`。小工具屆時會從 EU CDN 載入。