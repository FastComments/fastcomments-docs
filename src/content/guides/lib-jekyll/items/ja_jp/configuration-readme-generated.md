---
設定は3か所から来ます。後に指定されたソースが優先されます:

1. **グローバルのデフォルト** は `_config.yml` の `fastcomments:` キーの下にあります。
2. **ページコンテキスト** はページスコープのウィジェットに対して自動的に導出されます（下記参照）。
3. **タグ属性** はタグ自体に書かれた属性です。

つまりタグ上の `url_id` はページ由来の値を上書きし、それがグローバルのデフォルトを上書きします。

### 属性の構文

属性は `snake_case` の `key=value` ペアです:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **引用された** 値 (`"..."` または `'...'`) はそのままの文字列です。
- **引用符なしの** `true`/`false` はブール値になり、数値は数値になります。
- **引用符なしの** その他の値はページコンテキストの Liquid 変数として解決されます。例: `url_id=page.slug`。 (Liquid はタグの属性内で `{% raw %}\{{ ... }}{% endraw %}` を展開しないため、`"{% raw %}\{{ page.slug }}{% endraw %}"` ではなく生の `page.slug` 形式を使ってください。)

Snake_case の属性および設定キーは自動的に FastComments が期待する camelCase キーにマッピングされます（`tenant_id` → `tenantId`、`url_id` → `urlId`、`page_title` → `pageTitle`、`has_dark_background` → `hasDarkBackground`、など）。ウィジェットの他のオプションも [ウィジェット構成](https://docs.fastcomments.com/guide-customizations-and-configuration.html) からのものは同様にそのまま渡されます。

### ページ由来の値

ページスコープのウィジェット（`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`）については、明示的に設定しない限り、現在のページから自動的にこれらが入力されます:

- `url_id` ← `page.url`（訪問しているドメインに依存しない安定した識別子）
- `url` ← `site.url` + `page.url`（`url` が `_config.yml` に設定されている場合のみ）
- `page_title` ← `page.title`

サイト全体のウィジェット（最近のコメント/ディスカッション、トップページ、レビューの概要、ユーザーアクティビティフィード、バルクカウント）はページに紐づかず、これらを導出しません。

### EU データレジデンシー

EU のお客様は `region: eu` を追加します。グローバルに設定する場合:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

またはタグごとに: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`。ウィジェットはその場合 EU の CDN から読み込まれます。
---