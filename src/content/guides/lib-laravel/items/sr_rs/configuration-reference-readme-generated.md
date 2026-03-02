| Key | Env Variable | Default | Description |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID вашег FastComments tenant-а |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API кључ за позиве са сервера |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (САД) или `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Омогући SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` или `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login URL (ако није подешен, користи Laravel route) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout URL (ако није подешен, користи Laravel route) |
| `widget_defaults` | — | `[]` | Подразумеване опције конфигурације виџета |