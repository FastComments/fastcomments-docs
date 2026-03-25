## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | seq[string] | No |  |
| sso | string | No |  |
| searchSection | string | No |  |

## 回應

回傳: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## 範例

[inline-code-attrs-start title = 'searchUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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