| Ključ | Podrazumevano | Opis |
|---|---|---|
| `TENANT_ID` | `""` | Vaš FastComments tenant ID (`demo` za testiranje). |
| `API_KEY` | `""` | Vaš API tajni ključ. Potpisuje Secure SSO i autentificira `admin()`. |
| `REGION` | `None` | `None` za SAD, `"eu"` za EU regiju. |
| `SSO.ENABLED` | `False` | Uključite SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ili `"simple"` (nepotpisano). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Prikazuje se odjavljenim posjetiteljima; podrazumijevano na `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments polje do korisničkog atributa/puteva/funkcije. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ili putu s tačkama. |
| `SSO.USER_MAPPER` | `None` | Put s tačkama do `callable(user) -> dict`; najviši prioritet. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracija spajana u svaki widget (camelCase ključevi). |