## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| usernameStartsWith | string | Nein |  |
| mentionGroupIds | seq[string] | Nein |  |
| sso | string | Nein |  |
| searchSection | string | Nein |  |

## Antwort

Gibt zurück: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Beispiel

[inline-code-attrs-start title = 'searchUsers Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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