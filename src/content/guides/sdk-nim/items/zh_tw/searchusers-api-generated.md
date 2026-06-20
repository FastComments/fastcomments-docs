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

回傳: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## 範例

[inline-code-attrs-start title = 'searchUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/top-story",
  usernameStartsWith = "",
  mentionGroupIds = @[],
  sso = "",
  searchSection = ""
)

if response.isSome:
  let searchResult = response.get()
  echo "SearchUsersResult:", searchResult
else:
  echo "No result or error. HTTP response:", httpResponse
[inline-code-end]

---