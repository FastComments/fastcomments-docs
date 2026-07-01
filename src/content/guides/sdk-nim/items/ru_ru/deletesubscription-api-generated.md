## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| userId | string = "" | Нет |  |

## Ответ

Возвращает: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.deleteSubscription(
  tenantId = "my-tenant-123",
  id = "sub-789",
  userId = ""
)

if maybeResp.isSome:
  let apiResult = maybeResp.get()
  # использовать apiResult по мере необходимости
[inline-code-end]