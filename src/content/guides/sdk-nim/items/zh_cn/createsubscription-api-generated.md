## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | No |  |

## 响应

返回: [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## 示例

[inline-code-attrs-start title = 'createSubscription 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createSubscription(
  tenantId = "my-tenant-123",
  createAPIUserSubscriptionData = default(CreateAPIUserSubscriptionData),
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]

---