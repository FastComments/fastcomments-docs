FastComments는 세 가지 인증 모드를 지원합니다:

1. **익명** -- SSO 토큰 없음; 사용자는 세션 기반 식별자를 받습니다
2. **간단한 SSO** -- 데모 및 테스트용 클라이언트 측 토큰(보안 아님)
3. **보안 SSO** -- 프로덕션용 서버 서명 토큰

### 간단한 SSO

데모 및 로컬 테스트에 유용합니다. 간단한 SSO는 누구나 임의의 사용자를 가장할 수 있으므로 프로덕션 환경에서는 사용하지 마세요.

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData`는 또한 선택적 필드를 지원합니다:

- `id` -- 사용자 ID (설정하지 않으면 이메일을 기본값으로 사용)
- `displayName` -- 별도의 표시 이름
- `displayLabel` -- 이름 옆에 표시되는 사용자 지정 레이블(예: "VIP")
- `websiteUrl` -- 사용자 이름에 대한 링크
- `locale` -- 로케일 코드
- `isProfileActivityPrivate` -- 프로필 활동 숨기기(기본값: true)

### 보안 SSO

프로덕션에서는 백엔드가 API 시크릿을 사용하여 서명된 SSO 토큰을 생성합니다. iOS 앱은 이 토큰을 서버에서 가져와서 config에 전달합니다.

**백엔드에서** (FastComments Swift SDK를 사용하거나 다른 언어로):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// 이 토큰을 API를 통해 iOS 앱으로 반환하세요
```

**iOS 앱에서:**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // 백엔드에서 토큰을 가져옵니다
            let token = try? await fetchSSOTokenFromYourBackend()
            // 토큰으로 새 구성(config)을 생성하거나 로드 전에 설정하세요
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData`는 추가 필드를 지원합니다:

- `optedInNotifications` -- 이메일 알림 수신 동의
- `displayLabel` -- 사용자 지정 레이블
- `displayName` -- 표시 이름
- `websiteUrl` -- 웹사이트 URL
- `groupIds` -- 그룹 소속
- `isAdmin` -- 관리자 권한
- `isModerator` -- 중재자 권한
- `isProfileActivityPrivate` -- 프로필 공개 여부

---
---