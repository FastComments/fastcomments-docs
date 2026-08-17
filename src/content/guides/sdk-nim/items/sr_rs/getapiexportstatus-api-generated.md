## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetApiExportStatusOptions | No |  |

## Одговор

Враћа: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getApiExportStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (exportStatusOpt, httpResp) = client.getApiExportStatus(
  tenantId = "my-tenant-123",
  options = GetApiExportStatusOptions()
)
if exportStatusOpt.isSome:
  let exportStatus = exportStatusOpt.get()
  # користите exportStatus по потреби
[inline-code-end]