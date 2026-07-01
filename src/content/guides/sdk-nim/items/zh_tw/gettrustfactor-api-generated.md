---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| options | GetTrustFactorOptions | 否 |  |

## 回應

返回：[`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## 範例

[inline-code-attrs-start title = 'getTrustFactor 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (trustOpt, httpResp) = client.getTrustFactor(tenantId = "my-tenant-123", options = GetTrustFactorOptions())
if trustOpt.isSome:
  let trust = trustOpt.get()
  discard trust
[inline-code-end]

---