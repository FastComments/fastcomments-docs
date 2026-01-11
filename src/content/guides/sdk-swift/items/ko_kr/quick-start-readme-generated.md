### 공개 API 사용

```swift
import FastCommentsSwift

// API 클라이언트 생성
let publicApi = PublicAPI()

// 페이지의 댓글 가져오기
do {
    let response = try await publicApi.getCommentsPublic(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Found \(response.comments?.count ?? 0) comments")
    for comment in response.comments ?? [] {
        print("Comment: \(comment.comment ?? "")")
    }
} catch {
    print("Error fetching comments: \(error)")
}
```

### 인증된 API 사용

```swift
import FastCommentsSwift

// API 키로 구성 생성
let defaultApi = DefaultAPI()
defaultApi.apiKey = "your-api-key"

// 인증된 API를 사용하여 댓글 가져오기
do {
    let response = try await defaultApi.getComments(
        tenantId: "your-tenant-id",
        urlId: "page-url-id"
    )

    print("Total comments: \(response.count ?? 0)")
    for comment in response.comments ?? [] {
        print("Comment ID: \(comment.id ?? ""), Text: \(comment.comment ?? "")")
    }
} catch {
    print("Error: \(error)")
}
```

### 인증을 위한 SSO 사용

#### 보안 SSO (프로덕션 권장)

```swift
import FastCommentsSwift

let apiKey = "your-api-key"

// 보안 SSO 사용자 데이터 생성 (서버 측에서만!)
let userData = SecureSSOUserData(
    id: "user-123",              // 사용자 ID
    email: "user@example.com",   // 이메일
    username: "johndoe",         // 사용자 이름
    avatar: "https://example.com/avatar.jpg" // 아바타 URL
)

// SSO 토큰 생성
do {
    let sso = try FastCommentsSSO.createSecure(apiKey: apiKey, secureSSOUserData: userData)
    let token = try sso.createToken()

    print("SSO Token: \(token ?? "")")
    // 이 토큰을 프론트엔드로 전달하여 인증에 사용
} catch {
    print("Error creating SSO token: \(error)")
}
```

#### 간단한 SSO (개발/테스트용)

```swift
import FastCommentsSwift

// 간단한 SSO 사용자 데이터 생성 (API 키 불필요)
let userData = SimpleSSOUserData(
    username: "johndoe",
    email: "user@example.com",
    avatar: "https://example.com/avatar.jpg"
)

// 간단한 SSO 토큰 생성
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
do {
    let token = try sso.createToken()
    print("Simple SSO Token: \(token ?? "")")
} catch {
    print("Error creating SSO token: \(error)")
}
```