| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Su ID de inquilino de FastComments (`demo` para pruebas). |
| `API_KEY` | `""` | Su secreto de API. Firma SSO seguro y autentica `admin()`. |
| `REGION` | `None` | `None` para EE. UU, `"eu"` para la región UE. |
| `SSO.ENABLED` | `False` | Activar SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) o `"simple"` (sin firmar). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Mostrado a los visitantes desconectados; por defecto a `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Campo FastComments al atributo/ruta/llamable del usuario. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` o ruta punteada. |
| `SSO.USER_MAPPER` | `None` | Ruta punteada a `callable(user) -> dict`; mayor precedencia. |
| `WIDGET_DEFAULTS` | `{}` | Configuración fusionada en cada widget (claves camelCase). |