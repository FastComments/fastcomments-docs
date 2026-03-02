| 鍵 | 環境變數 | 預設值 | 說明 |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | 您的 FastComments 租戶 ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | 用於伺服器端呼叫的 API 金鑰 |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) 或 `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | 啟用 SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` 或 `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | Login URL (falls back to Laravel route) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | Logout URL (falls back to Laravel route) |
| `widget_defaults` | — | `[]` | 預設 widget 設定選項 |
---