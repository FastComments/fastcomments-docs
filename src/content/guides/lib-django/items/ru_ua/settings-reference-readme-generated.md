| Ключ | По умолчанию | Описание |
|---|---|---|
| `TENANT_ID` | `""` | Ваш идентификатор арендатора FastComments (`demo` для тестирования). |
| `API_KEY` | `""` | Ваш секрет API. Подписывает Secure SSO и аутентифицирует `admin()`. |
| `REGION` | `None` | `None` для США, `"eu"` для региона ЕС. |
| `SSO.ENABLED` | `False` | Включить SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) или `"simple"` (без подписи). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Показывается незарегистрированным посетителям; по умолчанию `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Поле FastComments, указывающее на атрибут/путь/вызываемую функцию пользователя. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` или путь через точки. |
| `SSO.USER_MAPPER` | `None` | Путь через точки к `callable(user) -> dict`; имеет наивысший приоритет. |
| `WIDGET_DEFAULTS` | `{}` | Конфигурация, объединяемая с каждым виджетом (ключи в camelCase). |