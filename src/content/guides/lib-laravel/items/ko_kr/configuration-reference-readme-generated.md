---
| 키 | 환경 변수 | 기본값 | 설명 |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | 귀하의 FastComments 테넌트 ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | 서버 측 호출용 API 키 |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (미국) 또는 `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO 활성화 |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` 또는 `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | 로그인 URL (Laravel 라우트로 대체됨) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | 로그아웃 URL (Laravel 라우트로 대체됨) |
| `widget_defaults` | — | `[]` | 기본 위젯 구성 옵션 |
---