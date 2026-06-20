## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| value | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getSearchUsers Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---