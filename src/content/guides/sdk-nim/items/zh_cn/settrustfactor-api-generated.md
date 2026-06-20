---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| trustFactor | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[SetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_user_trust_factor_response.nim)

## 示例

[inline-code-attrs-start title = 'setTrustFactor 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setTrustFactor(userId = "user-9876", trustFactor = "high", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTk4NzYiLCJpYXQiOjE2MjQwMDAwMDB9.signature")
if response.isSome:
  let resultObj = response.get()
  echo resultObj
else:
  echo "No response received"
[inline-code-end]

---