---
사용자에 대한 보안 인증을 구현하세요:

```kotlin
// 사용자 데이터 생성(권장: 서버 측에서)
val userData = SecureSSOUserData(
    "user-id",
    "user@example.com",
    "User Name",
    "https://path-to-avatar.jpg"
)

// SSO 토큰 생성(서버에서 수행해야 합니다!)
val sso = FastCommentsSSO.createSecure("YOUR_API_KEY", userData)
val token = sso.prepareToSend()

// 구성에 추가
config.sso = token
```
---