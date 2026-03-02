| Key | Env Variable | Default | Description |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID vašeg FastComments tenanta |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API ključ za serverske pozive |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (SAD) ili `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Omogući SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ili `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL za prijavu (pada na Laravel rutu ako nije postavljeno) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL za odjavu (pada na Laravel rutu ako nije postavljeno) |
| `widget_defaults` | — | `[]` | Podrazumevane opcije konfiguracije widgeta |