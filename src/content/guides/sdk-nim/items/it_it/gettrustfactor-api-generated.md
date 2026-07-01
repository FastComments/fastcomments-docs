## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| options | GetTrustFactorOptions | No |  |

## Risposta

Restituisce: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getTrustFactor'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (trustOpt, httpResp) = client.getTrustFactor(tenantId = "my-tenant-123", options = GetTrustFactorOptions())
if trustOpt.isSome:
  let trust = trustOpt.get()
  discard trust
[inline-code-end]