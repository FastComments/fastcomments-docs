| Ključ | Podrazumevano | Opis |
|---|---|---|
| `TENANT_ID` | `""` | Vaš FastComments tenant ID (`demo` za testiranje). |
| `API_KEY` | `""` | Vaša API tajna. Potpisuje Secure SSO i autentifikuje `admin()`. |
| `REGION` | `None` | `None` za SAD, `"eu"` za EU region. |
| `SSO.ENABLED` | `False` | Uključite SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ili `"simple"` (nepotpisano). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Prikazano posetiocima koji su odjavljeni; podrazumevano na `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments polje do korisničkog atributa/putanje/funkcije. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ili putanja sa tačkama. |
| `SSO.USER_MAPPER` | `None` | Putanja sa tačkama do `callable(user) -> dict`; najviši prioritet. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracija spojena u svaki widget (camelCase ključevi). |