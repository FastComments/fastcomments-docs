| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Vaš FastComments tenant ID (`demo` za testiranje). |
| `API_KEY` | `""` | Vaš API tajni ključ. Potpisuje Secure SSO i autentificira `admin()`. |
| `REGION` | `None` | `None` za US, `"eu"` za EU regiju. |
| `SSO.ENABLED` | `False` | Uključite SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ili `"simple"` (nepotpisano). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Prikazano odjavljenim posjetiteljima; zadano na `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments polje na atribut/put/poziv korisnika. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ili točka‑odvojeni put. |
| `SSO.USER_MAPPER` | `None` | Točka‑odvojeni put do `callable(user) -> dict`; najviši prioritet. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracija spojena u svaki widget (ključevi u camelCase). |