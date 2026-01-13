## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| usernameStartsWith | string | Όχι |  |
| mentionGroupIds | seq[string] | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  usernameStartsWith = "jo",
  mentionGroupIds = @["editors", "sports-team"],
  sso = "sso-abc-456"
)

if response.isSome:
  let users = response.get()
  echo "Users found: ", users
else:
  echo "No users found; HTTP status: ", httpResponse.status
[inline-code-end]

---