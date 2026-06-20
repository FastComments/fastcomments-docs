---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## 예제

[inline-code-attrs-start title = 'getUserInternalProfile 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserInternalProfile(commentId = "cmt-2026-00042", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoibXl1c2VyIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
if response.isSome:
  let profile = response.get()
  discard profile
[inline-code-end]

---