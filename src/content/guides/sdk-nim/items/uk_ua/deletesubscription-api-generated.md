## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string | No |  |

## Відповідь

Повертає: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSubscription(tenantId = "my-tenant-123", id = "sub-98765", userId = "user-456")
if response.isSome:
  let deleteResp = response.get()
  echo "Delete subscription response received"
else:
  echo "No subscription response"
[inline-code-end]

---