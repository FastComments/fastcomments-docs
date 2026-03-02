---
| Кључ | Променљива окружења | Подразумевано | Опис |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID вашег FastComments тенанта |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API кључ за серверске позиве |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) или `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Омогућити SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` или `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL за пријављивање (подразумевано користи Laravel руту) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL за одјаву (подразумевано користи Laravel руту) |
| `widget_defaults` | — | `[]` | Подразумеване опције конфигурације видгета |
---