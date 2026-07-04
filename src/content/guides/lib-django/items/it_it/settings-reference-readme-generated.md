| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Il tuo tenant ID di FastComments (`demo` per i test). |
| `API_KEY` | `""` | Il tuo segreto API. Firma Secure SSO e autentica `admin()`. |
| `REGION` | `None` | `None` per gli Stati Uniti, `"eu"` per la regione UE. |
| `SSO.ENABLED` | `False` | Attiva SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) o `"simple"` (non firmato). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Mostrato ai visitatori non autenticati; predefinito a `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Campo FastComments all'attributo/percorso/funzione dell'utente. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` o percorso puntato. |
| `SSO.USER_MAPPER` | `None` | Percorso puntato a `callable(user) -> dict`; precedenza più alta. |
| `WIDGET_DEFAULTS` | `{}` | Configurazione unita a ogni widget (chiavi camelCase). |