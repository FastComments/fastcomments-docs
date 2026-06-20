## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| batchJobId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## 範例

[inline-code-attrs-start title = 'getApiExportStatus 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiExportStatus(batchJobId = "export-job-2026-06-01", sso = "sso-abc123token")
if response.isSome:
  let exportStatus = response.get()
  echo repr(exportStatus)
else:
  echo "No export status available, HTTP code: ", httpResponse.statusCode
[inline-code-end]

---