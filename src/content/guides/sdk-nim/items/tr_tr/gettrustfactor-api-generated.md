## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## Örnek

[inline-code-attrs-start title = 'getTrustFactor Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTrustFactor(userId = "user-1001", sso = "sso-token-6f7d9c")
if response.isSome:
  let trust = response.get()
  echo "Received trust factor for user-1001"
else:
  echo "No trust factor returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---