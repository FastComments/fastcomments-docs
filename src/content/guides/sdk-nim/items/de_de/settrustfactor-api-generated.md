## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| options | SetTrustFactorOptions | Nein |  |

## Antwort

Rückgabe: [`Option[SetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_user_trust_factor_response.nim)

## Beispiel

[inline-code-attrs-start title = 'setTrustFactor Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = SetTrustFactorOptions(userId = "user-456", trustFactor = 5, reason = "spam detection")
let (trustResponse, httpResponse) = client.setTrustFactor(tenantId = "my-tenant-123", options = opts)
if trustResponse.isSome:
  let result = trustResponse.get()
[inline-code-end]

---