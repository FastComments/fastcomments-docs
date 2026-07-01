## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| createAPISSOUserData | CreateAPISSOUserData | Nie |  |

## Odpowiedź

Zwraca: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład addSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = CreateAPISSOUserData())
if optResp.isSome:
  let userResp = optResp.get()
[inline-code-end]