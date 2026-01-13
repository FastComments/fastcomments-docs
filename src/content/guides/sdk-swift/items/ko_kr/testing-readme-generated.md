### 단위 테스트 실행

단위 테스트는 SSO 기능을 다룹니다:

```bash
swift test --filter SSOTests
```

### 통합 테스트 실행

통합 테스트는 환경 변수를 설정해야 합니다:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```