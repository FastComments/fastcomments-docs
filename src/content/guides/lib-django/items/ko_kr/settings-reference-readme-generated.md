| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | FastComments 테넌트 ID (`demo`는 테스트용). |
| `API_KEY` | `""` | API 비밀 키. Secure SSO에 서명하고 `admin()`을 인증합니다. |
| `REGION` | `None` | `None`은 미국, `"eu"`는 EU 지역을 의미합니다. |
| `SSO.ENABLED` | `False` | SSO를 켭니다. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) 또는 `"simple"` (서명되지 않음). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | 로그아웃된 방문자에게 표시됩니다; 기본값은 `reverse("login"/"logout")`입니다. |
| `SSO.USER_MAP` | id/email/username | FastComments 필드가 사용자 속성/경로/콜러블에 매핑됩니다. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` 또는 점 표기 경로. |
| `SSO.USER_MAPPER` | `None` | `callable(user) -> dict`에 대한 점 표기 경로; 가장 높은 우선순위. |
| `WIDGET_DEFAULTS` | `{}` | 모든 위젯에 병합되는 구성 (camelCase 키). |