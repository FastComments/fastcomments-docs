## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | PostApiExportOptions | No |  |

## Risposta

Restituisce: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## Esempio

[inline-code-attrs-start title = 'postApiExport Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optExport, httpResp) = client.postApiExport(tenantId = "my-tenant-123", options = PostApiExportOptions())
if optExport.isSome:
  let export = optExport.get()
  echo export
[inline-code-end]