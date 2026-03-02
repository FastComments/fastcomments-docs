| Key | Env Variable | Default | Description |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Your FastComments tenant ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API key for server-side calls |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) or `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Enable SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` or `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login URL (falls back to Laravel route) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout URL (falls back to Laravel route) |
| `widget_defaults` | — | `[]` | Default widget config options |
---