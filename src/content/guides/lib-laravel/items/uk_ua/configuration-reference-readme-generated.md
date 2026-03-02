---
| Ключ | Змінна середовища | За замовчуванням | Опис |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Ідентифікатор вашого орендаря FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-ключ для серверних викликів |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (США) або `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Увімкнути SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` або `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL входу (за замовчуванням використовується маршрут Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL виходу (за замовчуванням використовується маршрут Laravel) |
| `widget_defaults` | — | `[]` | Параметри конфігурації віджету за замовчуванням |
---