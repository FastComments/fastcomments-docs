| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Your FastComments tenant ID (`demo` for testing). |
| `API_KEY` | `""` | Your API secret. Signs Secure SSO and authenticates `admin()`. |
| `REGION` | `None` | `None` for US, `"eu"` for the EU region. |
| `SSO.ENABLED` | `False` | Turn SSO on. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) or `"simple"` (unsigned). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Shown to signed-out visitors; default to `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments field to user attribute/path/callable. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` or dotted path. |
| `SSO.USER_MAPPER` | `None` | Dotted path to `callable(user) -> dict`; highest precedence. |
| `WIDGET_DEFAULTS` | `{}` | Config merged into every widget (camelCase keys). |