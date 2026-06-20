## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| value | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getSearchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---