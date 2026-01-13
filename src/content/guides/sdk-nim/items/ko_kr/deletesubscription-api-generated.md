## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteSubscription 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSubscription(tenantId = "my-tenant-123", id = "sub-98765", userId = "user-456")
if response.isSome:
  let deleteResp = response.get()
  echo "Delete subscription response received"
else:
  echo "No subscription response"
[inline-code-end]

---