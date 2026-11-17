1つのmigrate-pagesジョブからのすべてのマイグレーションを新しいものから順に元に戻すバッチジョブをキューに入れます。90日間利用可能です。
マイグレーション以降に変更されたレコードはそのまま残されるため、後の編集が上書きされることはありません。
復元/スキップされた数を確認するには、GET /api/v1/batch-jobs/{batchJobId} をポーリングしてください。

## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| undoId | string | はい |  |

## レスポンス

戻り値: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'undoPageMigrations の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]