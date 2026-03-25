## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| usernameStartsWith | string | 否 |  |
| mentionGroupIds | seq[string] | 否 |  |
| sso | string | 否 |  |
| searchSection | string | 否 |  |

## 响应

返回: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## 示例

[inline-code-attrs-start title = 'searchUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/2026/ai-product-launch",
  usernameStartsWith = "",
  mentionGroupIds = @[],
  sso = "",
  searchSection = ""
)
if response.isSome:
  let users = response.get()
  echo "Received users:", users.toString()
[inline-code-end]

---