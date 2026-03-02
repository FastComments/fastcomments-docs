---
| Chave | Variável de Ambiente | Padrão | Descrição |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Seu ID de tenant do FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Chave de API para chamadas do lado do servidor |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (EUA) ou `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Ativar SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ou `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL de login (usa a rota do Laravel como fallback) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL de logout (usa a rota do Laravel como fallback) |
| `widget_defaults` | — | `[]` | Opções de configuração padrão do widget |
---