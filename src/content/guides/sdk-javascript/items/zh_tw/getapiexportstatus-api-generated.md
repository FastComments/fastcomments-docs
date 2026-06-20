## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| batchJobId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## 範例

[inline-code-attrs-start title = 'getApiExportStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---