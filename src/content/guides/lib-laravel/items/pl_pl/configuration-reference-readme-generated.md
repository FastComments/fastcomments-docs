| Klucz | Zmienna środowiskowa | Domyślne | Opis |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Twój identyfikator tenantu FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Klucz API do wywołań po stronie serwera |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (USA) lub `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Włącz SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` lub `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL logowania (w przeciwnym razie używana jest trasa Laravela) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL wylogowania (w przeciwnym razie używana jest trasa Laravela) |
| `widget_defaults` | — | `[]` | Domyślne opcje konfiguracji widgetu |