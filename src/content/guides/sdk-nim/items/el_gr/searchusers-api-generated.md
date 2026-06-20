## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | seq[string] | No |  |
| sso | string | No |  |
| searchSection | string | No |  |

## Απόκριση

Επιστρέφει: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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