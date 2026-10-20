---
名前はドメインマイグレーションツールですが、個々のページ間でコメントを移行することもできます。

単に、古い `from` と新しい `to` の入力フィールドに、[urlId](/guide-customizations-and-configuration.html#url-id) の値を入力してください。

### API を使用したページの移行

多数のページを一度に移行するには、[`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) を使用します。リクエストごとに最大 100 組の `from` と `to` を受け取り、バッチジョブとして実行します。

`type` を `urlId` に設定すると、`urlId` が正確に `from` と一致するページだけが移動されるため、`post-1` は `post-10` と一致しません。ダッシュボードツールにはこのオプションはありません。

各 API 移行は、[`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) を使用して 90 日間元に戻すことができます。

---