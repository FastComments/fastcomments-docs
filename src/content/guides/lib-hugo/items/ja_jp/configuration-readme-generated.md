すべての FastComments ウィジェットのオプションは `hugo.toml` の `[params.fastcomments]` に設定され、ページごとに front matter の `[fastcomments]` で上書きできます。優先順位（低い -> 高い）: サイト params、ページ front matter、ショートコードのパラメータ。

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

`url` も `urlId` も指定されていない場合、コメントスレッドが安定した URL に紐づくように、`url` はページのパーマリンクをデフォルト値として使用します。

### EU データ居住

EU のお客様はウィジェットを `cdn-eu.fastcomments.com` にルーティングするために `region = "eu"` を設定します:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### キーの大文字小文字に関する注意

Hugo は `hugo.toml` と front matter のすべてのキーを小文字に変換しますが、FastComments ウィジェットは camelCase のキー（`tenantId`, `hasDarkBackground`）を必要とします。このコンポーネントは既知のトップレベルオプションの正しい大文字小文字を自動で復元するため、オプションは通常の camelCase 形式で記述してください。オブジェクト値の内部にネストされたキー（例えば `translations` マップのキーや `pageReactConfig` のフィールド）は復元されません。それらは代わりに FastComments ダッシュボードのカスタマイズ UI で設定してください。