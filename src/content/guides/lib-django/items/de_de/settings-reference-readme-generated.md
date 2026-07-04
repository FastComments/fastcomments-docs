| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Ihre FastComments Mandanten‑ID (`demo` für Tests). |
| `API_KEY` | `""` | Ihr API‑Geheimnis. Signiert Secure SSO und authentifiziert `admin()`. |
| `REGION` | `None` | `None` für US, `"eu"` für die EU‑Region. |
| `SSO.ENABLED` | `False` | SSO aktivieren. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) oder `"simple"` (unsigned). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Wird abgemeldeten Besuchern angezeigt; standardmäßig `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments‑Feld zum Benutzerattribut/Pfad/Callable. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` oder gepunkteter Pfad. |
| `SSO.USER_MAPPER` | `None` | Gepunkteter Pfad zu `callable(user) -> dict`; höchste Priorität. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguration, die in jedes Widget (camelCase‑Schlüssel) zusammengeführt wird. |