| Clave | Variable de entorno | Predeterminado | Descripción |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Su ID de inquilino de FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Clave API para llamadas del lado del servidor |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (EE. UU.) o `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Habilitar SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` o `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL de inicio de sesión (usa la ruta de Laravel como alternativa) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL de cierre de sesión (usa la ruta de Laravel como alternativa) |
| `widget_defaults` | — | `[]` | Opciones de configuración predeterminadas del widget |