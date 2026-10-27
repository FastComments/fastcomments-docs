A `BatchJob` は、インポート、エクスポート、または[ページマイグレーション](#pages-migrations-post) のような長時間実行されるタスクです。ジョブはキューに入れられ、バックグラウンドで取得され実行されます。

`processedCount` と `totalCount` は進捗を示します。その単位は `type` に依存します。

- `migrate-pages`: 要求されたマイグレーション数に対する完了したマイグレーション数。
- `undo-page-migration`: 復元されたレコード数。`totalCount` はありません。
- `import`、`export`、その他のジョブ: 処理された行数。

`BatchJob` オブジェクトの構造は以下の通りです。

[inline-code-attrs-start title = 'BatchJob の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** 例: migrate-pages、undo-page-migration、migrate-domains、import、export、users-export、または copy-comments。 **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** 事前に分かっている総作業量。 **/
    totalCount?: number
    /** migrate-pages: 完了した各マイグレーションごとに1つのエントリ、リクエスト順。 **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: ジョブが完了したら設定されます（成功または失敗）。90日以内に POST /api/v1/pages/migrations/:undoId/undo に渡してください。
     * undo-page-migration: 元に戻すマイグレーションです。
     **/
    undoId?: string
    /** undo-page-migration: ジョブが完了したら設定されます。 **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** マイグレーション前の値に戻されたレコード。 **/
    restored: number
    /** マイグレーション後に変更された、または存在しなくなったためそのまま残されたレコード。 **/
    skipped: number
    /** ページ統合中に削除されたマイグレーションのレコード。これらは復元できません。 **/
    notRestorable: number
}
[inline-code-end]

#### リトライ

ページマイグレーションジョブを実行しているサーバーが再起動した場合、ジョブはキューに戻され、最後に完了したマイグレーションの後から再開されます。ジョブは最大 3 回まで試行され、それ以上は `status` が `failed` になります。