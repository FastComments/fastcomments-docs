---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Ні |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`Option[UpdateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_subscription_api_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateSubscription(
  tenantId = "my-tenant-123",
  id = "sub-456",
  updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(
    subscribed = true,
    channels = @["email", "push"]
  ),
  userId = "user-789"
)

if response.isSome:
  let updated = response.get()
  echo "Subscription updated:", updated
else:
  echo "Update failed, HTTP response:", httpResponse
[inline-code-end]

---