---
| Schlüssel | Umgebungsvariable | Standard | Beschreibung |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Ihre FastComments Tenant-ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | API-Schlüssel für serverseitige Aufrufe |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) oder `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO aktivieren |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` oder `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login-URL (fällt auf die Laravel-Route zurück) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout-URL (fällt auf die Laravel-Route zurück) |
| `widget_defaults` | — | `[]` | Standard-Widget-Konfigurationsoptionen |
---