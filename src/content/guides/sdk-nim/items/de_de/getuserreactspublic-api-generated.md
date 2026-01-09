## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | seq[string] | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[GetUserReactsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_reacts_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'getUserReactsPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(tenantId = "my-tenant-123", postIds = @[], sso = "")
if response.isSome:
  let reacts = response.get()
  discard reacts
[inline-code-end]

---