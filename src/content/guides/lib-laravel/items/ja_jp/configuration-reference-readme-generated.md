| キー | 環境変数 | デフォルト | 説明 |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | あなたの FastComments テナント ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | サーバーサイドの呼び出し用 API キー |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (米国) または 'eu' |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO を有効にする |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` または `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | ログイン URL (Laravel のルートにフォールバックします) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | ログアウト URL (Laravel のルートにフォールバックします) |
| `widget_defaults` | — | `[]` | ウィジェットのデフォルト設定オプション |