| Clé | Variable d'environnement | Par défaut | Description |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Votre ID de locataire FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Clé API pour les appels côté serveur |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) ou `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Activer SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ou `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL de connexion (retombe sur la route Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL de déconnexion (retombe sur la route Laravel) |
| `widget_defaults` | — | `[]` | Options de configuration par défaut du widget |