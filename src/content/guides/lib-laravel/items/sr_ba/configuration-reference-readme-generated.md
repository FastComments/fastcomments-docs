| Ključ | Env varijabla | Zadano | Opis |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Vaš FastComments tenant ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API ključ za server-side pozive |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (SAD) ili `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Omogući SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ili `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL za prijavu (u suprotnom koristi Laravel rutu) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL za odjavu (u suprotnom koristi Laravel rutu) |
| `widget_defaults` | — | `[]` | Podrazumijevane opcije konfiguracije widgeta |
---