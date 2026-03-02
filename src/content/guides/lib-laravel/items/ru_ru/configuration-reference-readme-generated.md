| Ключ | Переменная окружения | По умолчанию | Описание |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID вашего tenant в FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-ключ для серверных вызовов |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) или `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Включить SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` или `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL для входа (при отсутствии используется маршрут Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL для выхода (при отсутствии используется маршрут Laravel) |
| `widget_defaults` | — | `[]` | Параметры конфигурации виджета по умолчанию |