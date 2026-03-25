## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| usernameStartsWith | string | Non |  |
| mentionGroupIds | seq[string] | Non |  |
| sso | string | Non |  |
| searchSection | string | Non |  |

## Réponse

Renvoie : [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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