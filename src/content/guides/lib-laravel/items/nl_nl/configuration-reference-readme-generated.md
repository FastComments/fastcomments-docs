| Sleutel | Omgevingsvariabele | Standaard | Beschrijving |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Uw FastComments tenant-ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-sleutel voor server-side-oproepen |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (VS) of `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO inschakelen |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` of `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login-URL (valt terug op Laravel-route) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout-URL (valt terug op Laravel-route) |
| `widget_defaults` | — | `[]` | Standaard widgetconfiguratie-opties |