## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| batchJobId | string | 任意 |  |
| sso | string | 任意 |  |

## レスポンス

戻り値: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'getApiExportStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---