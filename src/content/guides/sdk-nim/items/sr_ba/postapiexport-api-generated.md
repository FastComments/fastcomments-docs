## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | PostApiExportOptions | Ne |  |

## Odgovor

Vraća: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## Primer

[inline-code-attrs-start title = 'postApiExport Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optExport, httpResp) = client.postApiExport(tenantId = "my-tenant-123", options = PostApiExportOptions())
if optExport.isSome:
  let export = optExport.get()
  echo export
[inline-code-end]