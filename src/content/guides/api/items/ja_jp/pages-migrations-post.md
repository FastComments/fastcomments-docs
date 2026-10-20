[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

このルートは、`urlId` またはドメイン単位でコメントとページを別の場所へ移動します。ダッシュボードの [Domain Migration tool](/guide-migrations.html#migrating-domains) と同様の機能です。1 回のリクエストで最大 100 件のマイグレーションを送信できます。指定された順序で実行されるため、`a` から `b`、続いて `b` から `c` へのマイグレーションが可能です。

各マイグレーションには `type` が存在します。

- `urlId` は、`urlId` が正確に `from` と一致するページを移動します。`post-1` は `post-10` と一致しません。
- `domain` は、すべての `urlId` と `url` 内の `from` を検索/置換し、各コメントの `domain` を更新します。両方の値は `old.example.com` のようなドメインである必要があります。

コメント、ページ、ページリアクション、投票、通知、購読すべてが移動します。`to` にすでにページが存在する場合、2 つのページはマージされます。

マイグレーションはバッチジョブとして実行されます。レスポンスには `batchJobId` が含まれます。[`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) をポーリングして進行状況を確認してください。ジョブが完了すると、各マイグレーションの結果と、90 日以内に [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) に渡せる `undoId` が返されます。

リクエストはキューに入る前に検証されます。いずれかのマイグレーションが無効な場合、全体のリクエストは `422` エラーで失敗し、何も実行されません。

マイグレーションのキューイング自体には費用がかかりません。ジョブが完了すると、移行された 1000 件のコメントごとに 1 クレジットが消費され、最低でも 1 クレジットが必要です。

[inline-code-attrs-start title = 'ページマイグレーション cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'ページマイグレーション リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** 指定された順序で実行されます。最大 100 件です。 **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** 最大 1000 文字です。前後の空白は削除されます。 **/
    from: string
    /** "from" と異なる必要があります。最大 1000 文字です。 **/
    to: string
    /** "urlId" は正確な urlId のみ一致します。"domain" はすべての urlId と url 内の値を検索/置換します。 **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'ページマイグレーション 応答構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** 失敗時に含まれます。invalid-migration の場合、最初の無効なマイグレーションのインデックスを示します。 **/
    reason?: string
    /** 進捗と結果を取得するために GET /api/v1/batch-jobs/:id に渡します。 **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- 1 つのテナントのジョブは同時に 1 つずつ、古いものから順に実行されます。2 番目のマイグレーションは最初のジョブが完了するまで待機します。
- `PATCH /api/v1/pages/:id` に新しい `urlId` を指定すると、ページのみが変更され、コメントは変更されません。このルートを使用してページのコメントを移動します。