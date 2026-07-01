## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Rückgabe: [`Option[GetSSOUserByIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_id_api_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getSSOUserById Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserById(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let userInfo = response.get()
  discard userInfo
[inline-code-end]