| Ключ | Переменная окружения | Значение по умолчанию | Описание |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID вашего FastComments tenant'а |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-ключ для серверных вызовов |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (США) или `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Включить SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` или `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL входа (используется маршрут Laravel по умолчанию) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL выхода (используется маршрут Laravel по умолчанию) |
| `widget_defaults` | — | `[]` | Параметры конфигурации виджета по умолчанию |