## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| sure | string | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")
if response.isSome:
  let emptyResp = response.get()
else:
  discard httpResponse
[inline-code-end]