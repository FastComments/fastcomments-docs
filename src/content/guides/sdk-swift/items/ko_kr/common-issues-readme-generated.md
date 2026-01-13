### 401 인증 오류

인증된 API를 사용할 때 401 오류가 발생하면:

1. **API 키 확인**: FastComments 대시보드에서 올바른 API 키를 사용하고 있는지 확인하세요
2. **테넌트 ID 확인**: 테넌트 ID가 계정과 일치하는지 확인하세요
3. **API 키 형식**: API 키는 API 클라이언트에 설정되어야 합니다:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **잘못된 API 사용 중인지 확인**: 인증 호출에는 `DefaultAPI`( `PublicAPI`가 아님)를 사용하고 있는지 확인하세요

### SSO 토큰 문제

SSO 토큰이 작동하지 않는 경우:

1. **프로덕션에서는 보안 모드 사용**: 프로덕션에서는 항상 `FastCommentsSSO.createSecure()`를 API 키와 함께 사용하세요
2. **서버 측에서만 생성**: 보안 SSO 토큰은 서버에서 생성하고, API 키를 클라이언트에 노출하지 마세요
3. **사용자 데이터 확인**: 필수 필드(id, email, username)가 모두 제공되었는지 확인하세요
4. **토큰 만료**: 보안 SSO 토큰에는 타임스탬프가 포함되어 만료될 수 있습니다. 필요에 따라 새 토큰을 생성하세요.

### SSL/TLS 오류

SSL/TLS 오류가 발생하면:

1. 앱의 Info.plist가 fastcomments.com에 대한 HTTPS 연결을 허용하는지 확인하세요
2. 연결을 차단할 수 있는 App Transport Security 예외를 사용하고 있지 않은지 확인하세요