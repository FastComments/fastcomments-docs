## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | seq[string] | Nej |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(
  tenantId = "my-tenant-123",
  postIds = @["news/article-2026", "blog/opinion-987"],
  sso = ""
)
if response.isSome:
  let reacts = response.get()
  echo "Received user reacts for tenant: ", "my-tenant-123"
[inline-code-end]

---