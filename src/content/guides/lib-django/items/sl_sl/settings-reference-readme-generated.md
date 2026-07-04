| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Vaš FastComments identifikator najemnika (`demo` za testiranje). |
| `API_KEY` | `""` | Vaše API skrivnost. Podpisuje Secure SSO in avtenticira `admin()`. |
| `REGION` | `None` | `None` za ZDA, `"eu"` za EU regijo. |
| `SSO.ENABLED` | `False` | Omogoči SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ali `"simple"` (nepodpisano). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Prikazano obiskovalcem, ki so odjavljeni; privzeto na `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments polje do uporabniškega atributa/poti/klicne funkcije. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ali pot z ločnicami. |
| `SSO.USER_MAPPER` | `None` | Pot z ločnicami do `callable(user) -> dict`; najvišja prednost. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracija, združena v vsak widget (ključi v camelCase). |