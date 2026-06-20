## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| batchJobId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## 예제

[inline-code-attrs-start title = 'getApiExportStatus 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiExportStatus(batchJobId = "export-job-2026-06-01", sso = "sso-abc123token")
if response.isSome:
  let exportStatus = response.get()
  echo repr(exportStatus)
else:
  echo "No export status available, HTTP code: ", httpResponse.statusCode
[inline-code-end]

---