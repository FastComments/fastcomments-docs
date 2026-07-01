## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| options | GetTrustFactorOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTrustFactor Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (trustOpt, httpResp) = client.getTrustFactor(tenantId = "my-tenant-123", options = GetTrustFactorOptions())
if trustOpt.isSome:
  let trust = trustOpt.get()
  discard trust
[inline-code-end]

---