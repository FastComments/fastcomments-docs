## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Нет |  |

## Ответ

Возвращает: [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createData = CreateAPIUserSubscriptionData(
  subscriberId = "user-987",
  email = "jane.doe@newsreader.com",
  urlId = "news/local-weather",
  active = true,
  tags = @["weather", "local"],
  frequency = "immediate"
)
let (response, httpResponse) = client.createSubscription(tenantId = "my-tenant-123", createAPIUserSubscriptionData = createData)
if response.isSome:
  let created = response.get()
  echo "Created subscription id: ", created.id
[inline-code-end]

---