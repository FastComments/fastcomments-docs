## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回: [`Option[GetVotes_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes200response.nim)

## 示例

[inline-code-attrs-start title = 'getVotes 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/2026/major-update")
if response.isSome:
  let votes = response.get()
  discard votes
else:
  discard httpResponse
[inline-code-end]

---