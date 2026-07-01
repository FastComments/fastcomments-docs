## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Ні |  |

## Відповідь

Повертає: [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## Приклад

[inline-code-attrs-start title = 'createSubscription Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createSubscription(
  tenantId = "my-tenant-123",
  createAPIUserSubscriptionData = default(CreateAPIUserSubscriptionData),
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]