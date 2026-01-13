## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | float64 | Нет |  |

## Ответ

Возвращает: [`Option[GetModerators_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getModerators'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Moderators fetched successfully"
  echo moderators
[inline-code-end]