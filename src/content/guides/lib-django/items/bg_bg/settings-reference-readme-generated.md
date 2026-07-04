| Ключ | По подразбиране | Описание |
|---|---|---|
| `TENANT_ID` | `""` | Вашият FastComments tenant ID (`demo` за тестване). |
| `API_KEY` | `""` | Вашият API secret. Подписва Secure SSO и удостоверява `admin()`. |
| `REGION` | `None` | `None` за САЩ, `"eu"` за региона на ЕС. |
| `SSO.ENABLED` | `False` | Включете SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) или `"simple"` (неподписан). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Показва се на посетители, които са излезли; по подразбиране е `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Поле на FastComments към атрибут/път/извикваема функция на потребителя. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` или точков път. |
| `SSO.USER_MAPPER` | `None` | Точков път към `callable(user) -> dict`; най-висок приоритет. |
| `WIDGET_DEFAULTS` | `{}` | Конфигурация, обединена във всеки уиджет (ключове в camelCase). |