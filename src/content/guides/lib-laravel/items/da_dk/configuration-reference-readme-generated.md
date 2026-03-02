| Key | Env Variable | Default | Description |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Dit FastComments tenant-ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-nøgle til server-side-opkald |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) eller `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Aktivér SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` eller `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login-URL (falder tilbage til Laravel-route) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout-URL (falder tilbage til Laravel-route) |
| `widget_defaults` | — | `[]` | Standardindstillinger for widgetkonfiguration |