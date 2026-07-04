| Ключ | За замовчуванням | Опис |
|---|---|---|
| `TENANT_ID` | `""` | Ваш FastComments tenant ID (`demo` для тестування). |
| `API_KEY` | `""` | Ваш API secret. Підписує Secure SSO і автентифікує `admin()`. |
| `REGION` | `None` | `None` для США, `"eu"` для регіону ЄС. |
| `SSO.ENABLED` | `False` | Увімкнути SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) або `"simple"` (не підписаний). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Показується відвідувачам, які вийшли; за замовчуванням `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Поле FastComments до атрибуту/шляху/виклику користувача. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` або шлях через крапки. |
| `SSO.USER_MAPPER` | `None` | Шлях через крапки до `callable(user) -> dict`; найвищий пріоритет. |
| `WIDGET_DEFAULTS` | `{}` | Конфіг, який зливається у кожен віджет (ключі у camelCase). |