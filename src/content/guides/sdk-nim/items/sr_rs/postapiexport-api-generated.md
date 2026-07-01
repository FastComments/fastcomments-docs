## Параметри

| Име   | Тип                     | Захтевано | Опис |
|-------|------------------------|-----------|------|
| tenantId | string                 | Yes       |  |
| options  | PostApiExportOptions   | No        |  |

## Одговор

Враћа: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## Пример

[inline-code-attrs-start title = 'postApiExport Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optExport, httpResp) = client.postApiExport(tenantId = "my-tenant-123", options = PostApiExportOptions())
if optExport.isSome:
  let export = optExport.get()
  echo export
[inline-code-end]