### 401 권한 없음 오류

인증된 API를 사용할 때 401 오류가 발생하는 경우:

1. **API 키 확인**: FastComments 대시보드에 있는 올바른 API 키를 사용하고 있는지 확인하세요
2. **테넌트 ID 확인**: 테넌트 ID가 계정과 일치하는지 확인하세요
3. **API 키 형식**: API 키는 Configuration에 전달되어야 합니다:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO 토큰 문제

SSO 토큰이 작동하지 않는 경우:

1. **프로덕션에서는 보안 모드 사용**: 프로덕션에서는 항상 `FastCommentsSSO::new_secure()`를 API 키와 함께 사용하세요
2. **서버 측에서만 생성**: SSO 토큰은 서버에서 생성하고 API 키를 클라이언트에 노출하지 마세요
3. **사용자 데이터 확인**: 필요한 모든 필드 (id, email, username)가 제공되었는지 확인하세요

### 비동기 런타임 오류

SDK는 비동기 작업에 tokio를 사용합니다. 다음을 확인하세요:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // 여기에 비동기 코드를 작성하세요
}
```