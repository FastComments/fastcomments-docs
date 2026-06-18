一度 `hugo.toml` にテナント ID を設定してください:

```toml
[params.fastcomments]
  tenantId = "demo"   # "demo" をあなたの FastComments テナント ID に置き換えてください
```

その後、コメントウィジェットをテーマに組み込む（[テーマ統合](#theme-integration-readme-generated)を参照）か、任意のページの Markdown にショートコードを挿入してください:

```text
\{{< fastcomments >}}
```