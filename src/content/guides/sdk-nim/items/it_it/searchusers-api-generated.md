## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | seq[string] | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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