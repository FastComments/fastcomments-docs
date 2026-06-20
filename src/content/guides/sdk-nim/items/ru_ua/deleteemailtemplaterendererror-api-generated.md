---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| errorId | string | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования deleteEmailTemplateRenderError'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplateRenderError(tenantId = "my-tenant-123", id = "welcome-email-template", errorId = "err-20250615-01")
if response.isSome:
  let emptyResp = response.get()
  echo "Deleted render error, tenant:", "my-tenant-123"
  echo "HTTP status:", httpResponse.status
else:
  echo "No body returned, HTTP status:", httpResponse.status
[inline-code-end]

---