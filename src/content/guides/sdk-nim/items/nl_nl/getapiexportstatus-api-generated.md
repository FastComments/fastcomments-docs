---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| batchJobId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getApiExportStatus Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiExportStatus(batchJobId = "export-job-2026-06-01", sso = "sso-abc123token")
if response.isSome:
  let exportStatus = response.get()
  echo repr(exportStatus)
else:
  echo "No export status available, HTTP code: ", httpResponse.statusCode
[inline-code-end]

---