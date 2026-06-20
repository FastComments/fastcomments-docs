## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | 任意 |  |
| sso | string | 任意 |  |

## レスポンス

返却値: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## 例

[inline-code-attrs-start title = 'getTrustFactor の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTrustFactor(userId = "user-1001", sso = "sso-token-6f7d9c")
if response.isSome:
  let trust = response.get()
  echo "Received trust factor for user-1001"
else:
  echo "No trust factor returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---