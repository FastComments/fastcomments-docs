## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|----------|
| tenantId | string | Да |  |
| options | GetApiExportStatusOptions | Нет |  |

## Ответ

Возвращает: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getApiExportStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (exportStatusOpt, httpResp) = client.getApiExportStatus(
  tenantId = "my-tenant-123",
  options = GetApiExportStatusOptions()
)
if exportStatusOpt.isSome:
  let exportStatus = exportStatusOpt.get()
  # использовать exportStatus по необходимости
[inline-code-end]

---