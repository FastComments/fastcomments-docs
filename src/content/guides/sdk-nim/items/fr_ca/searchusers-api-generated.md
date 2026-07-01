## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| options | SearchUsersOptions | Non |  |

## Réponse

Renvoie : [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = SearchUsersOptions()
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---