| Ključ | Varijabla okoline | Zadano | Opis |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Vaš FastComments tenant ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API ključ za pozive sa serverske strane |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (SAD) ili `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Omogući SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ili `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL za prijavu (u nedostatku koristi se Laravel ruta) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL za odjavu (u nedostatku koristi se Laravel ruta) |
| `widget_defaults` | — | `[]` | Zadane opcije konfiguracije widgeta |