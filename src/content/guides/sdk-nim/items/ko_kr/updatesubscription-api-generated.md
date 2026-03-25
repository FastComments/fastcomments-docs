---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | 아니오 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`Option[UpdateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_subscription_api_response.nim)

## 예제

[inline-code-attrs-start title = 'updateSubscription 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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