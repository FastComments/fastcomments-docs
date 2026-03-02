---
| Chiave | Variabile d'ambiente | Predefinito | Descrizione |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Il tuo ID tenant di FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Chiave API per chiamate lato server |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) o `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Abilita SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` o `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL di login (in alternativa usa la route di Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL di logout (in alternativa usa la route di Laravel) |
| `widget_defaults` | — | `[]` | Opzioni di configurazione predefinite del widget |
---