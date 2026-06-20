## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| usernameStartsWith | string | Hayır |  |
| mentionGroupIds | seq[string] | Hayır |  |
| sso | string | Hayır |  |
| searchSection | string | Hayır |  |

## Yanıt

Döndürür: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Örnek

[inline-code-attrs-start title = 'searchUsers Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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