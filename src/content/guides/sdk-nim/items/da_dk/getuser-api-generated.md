## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Respons

Returnerer: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getUser Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userOpt, httpResp) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if userOpt.isSome:
  let user = userOpt.get()
  echo user
else:
  echo "User not found"
[inline-code-end]