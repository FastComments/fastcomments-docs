## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetTrustFactorOptions | No |  |

## Respons

Retourneert: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTrustFactor Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (trustOpt, httpResp) = client.getTrustFactor(tenantId = "my-tenant-123", options = GetTrustFactorOptions())
if trustOpt.isSome:
  let trust = trustOpt.get()
  discard trust
[inline-code-end]