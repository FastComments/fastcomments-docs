佇列一個批次工作，以最新優先的順序反轉某個 migrate-pages 工作的每個遷移。可用期限為 90 天。
遷移之後有變更的記錄將保持不變，因此之後的編輯永遠不會被覆寫。
請輪詢 GET /api/v1/batch-jobs/{batchJobId} 以取得已還原/已略過的計數。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| undoId | string | 是 |  |

## 回應

回傳:[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'undoPageMigrations 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]