| Кључ | Подразумевано | Опис |
|---|---|---|
| `TENANT_ID` | `""` | Ваш FastComments tenant ID (`demo` за тестирање). |
| `API_KEY` | `""` | Ваш API тајн. Потписује Secure SSO и аутентификује `admin()`. |
| `REGION` | `None` | `None` за САД, `"eu"` за EU регион. |
| `SSO.ENABLED` | `False` | Укључите SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) или `"simple"` (непотписано). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Приказује се одјављеним посетиоцима; подразумевано је `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments поље у атрибут/пут/функцију корисника. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` или путања са тачкама. |
| `SSO.USER_MAPPER` | `None` | Путања са тачкама до `callable(user) -> dict`; највиши приоритет. |
| `WIDGET_DEFAULTS` | `{}` | Конфигурација спојена у сваки виџет (camelCase кључеви). |