| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Vaš FastComments tenant ID (`demo` za testiranje). |
| `API_KEY` | `""` | Vaša API tajna. Potpisuje Secure SSO i autentikuje `admin()`. |
| `REGION` | `None` | `None` za SAD, `"eu"` za EU regiju. |
| `SSO.ENABLED` | `False` | Uključite SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ili `"simple"` (nepotpisano). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Prikazano posjetiocima koji su odjavljeni; podrazumevano na `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments polje za korisnički atribut/put/poziv. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ili put sa tačkama. |
| `SSO.USER_MAPPER` | `None` | Put sa tačkama do `callable(user) -> dict`; najviši prioritet. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracija spajana u svaki widget (camelCase ključevi). |