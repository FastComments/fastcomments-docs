---
| 键 | 环境变量 | 默认值 | 描述 |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | 您的 FastComments 租户 ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | 用于服务器端调用的 API 密钥 |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) 或 `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | 启用 SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` 或 `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | 登录 URL (回退到 Laravel 路由) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | 登出 URL (回退到 Laravel 路由) |
| `widget_defaults` | — | `[]` | 默认 widget 配置选项 |
---