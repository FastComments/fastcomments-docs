| Ключ | Променлива на средата | По подразбиране | Описание |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Вашият FastComments tenant ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API ключ за повиквания от страна на сървъра |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) или `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Разрешаване на SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` или `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL за вход (в противен случай използва маршрута на Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL за изход (в противен случай използва маршрута на Laravel) |
| `widget_defaults` | — | `[]` | Опции за конфигурация на widget по подразбиране |
---