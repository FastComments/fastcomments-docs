[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

このルートは、1つの [`POST /api/v1/pages/migrations`](#pages-migrations-post) ジョブからのすべてのマイグレーションを元に戻します。完了したマイグレーションジョブから `undoId` を取得するには、[`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) を使用します。`undoId` は 90 日間使用可能で、1 回だけ使用できます。

元に戻す処理はマイグレーションと同様にバッチジョブとして実行され、レスポンスにはその `batchJobId` が含まれます。ジョブが完了すると、`pageMigrationUndoResult` が次のように報告します：

- `restored`: マイグレーション前の値に戻されたレコード。
- `skipped`: マイグレーション後に変更された、または存在しなくなったためそのまま残されたレコード。後の編集は上書きされません。
- `notRestorable`: 2 つのページをマージしたり、重複ページのリアクションを削除した際にマイグレーションで削除されたレコード。これらは復元できません。

マイグレーションは新しいものから順に逆転されるため、`a` から `b`、続いて `b` から `c` へのチェーンは `a` に戻ります。途中で失敗したマイグレーションジョブでも元に戻すことができ、失敗前に行われた変更を取り消します。

元に戻す処理をキューに入れることは無料です。ジョブが完了した際、復元されたレコード 1000 件ごとに `1` クレジットがかかります（最低 `1` クレジット）。

[inline-code-attrs-start title = 'ページマイグレーションの元に戻し cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'ページマイグレーションの元に戻し リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページマイグレーションの元に戻し レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * 失敗時に含まれます。
     * undo-not-found (404): この undoId を持つマイグレーションが存在しない、または 90 日以上前のものです。
     * already-undone (409) および undo-in-progress (409): マイグレーションは既に元に戻されているか、元に戻す処理がキューに入っているか実行中です。
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 進捗と結果を取得するために GET /api/v1/batch-jobs/:id に渡してください。 **/
    batchJobId?: string
}
[inline-code-end]