| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Dit FastComments tenant‑ID (`demo` til test). |
| `API_KEY` | `""` | Din API‑hemmelighed. Signerer Secure SSO og godkender `admin()`. |
| `REGION` | `None` | `None` for USA, `"eu"` for EU‑regionen. |
| `SSO.ENABLED` | `False` | Aktiver SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) eller `"simple"` (uden signatur). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Vises for brugere som er logget ud; standard er `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments‑felt til bruger‑attribut/sti/kaldbar. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` eller punktnotationsti. |
| `SSO.USER_MAPPER` | `None` | Punktnotationsti til `callable(user) -> dict`; højeste præcedens. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguration flettet ind i hver widget (camelCase‑nøgler). |