---
| Anahtar | Ortam Değişkeni | Varsayılan | Açıklama |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | FastComments kiracı kimliğiniz |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Sunucu tarafı çağrıları için API anahtarı |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (ABD) veya `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO'yu etkinleştir |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` veya `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Giriş URL'si (Laravel rotasına geri döner) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Çıkış URL'si (Laravel rotasına geri döner) |
| `widget_defaults` | — | `[]` | Varsayılan widget yapılandırma seçenekleri |
---