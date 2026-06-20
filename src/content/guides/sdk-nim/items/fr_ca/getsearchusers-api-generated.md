## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| value | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---