---
| Ključ | Spremenljivka okolja | Privzeto | Opis |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | ID vašega FastComments najemnika |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API ključ za klice na strežniški strani |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (ZDA) ali `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Omogoči SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ali `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL za prijavo (če ni nastavljen, uporabi Laravelovo pot) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL za odjavo (če ni nastavljen, uporabi Laravelovo pot) |
| `widget_defaults` | — | `[]` | Privzete možnosti konfiguracije widgeta |
---