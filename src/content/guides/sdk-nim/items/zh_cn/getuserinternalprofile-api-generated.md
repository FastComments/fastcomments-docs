## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## 示例

[inline-code-attrs-start title = 'getUserInternalProfile 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserInternalProfile(commentId = "cmt-2026-00042", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoibXl1c2VyIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
if response.isSome:
  let profile = response.get()
  discard profile
[inline-code-end]

---