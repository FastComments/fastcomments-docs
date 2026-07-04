---
| Sleutel | Standaard | Beschrijving |
|---|---|---|
| `TENANT_ID` | `""` | Uw FastComments-tenant-ID (`demo` voor testen). |
| `API_KEY` | `""` | Uw API‑geheim. Ondertekent Secure SSO en authenticeert `admin()`. |
| `REGION` | `None` | `None` voor US, `"eu"` voor de EU-regio. |
| `SSO.ENABLED` | `False` | Schakel SSO in. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) of `"simple"` (ongetekend). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Getoond aan uitgelogde bezoekers; standaard `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments‑veld naar gebruikersattribuut/pad/aanroepbaar. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` of puntnotatie‑pad. |
| `SSO.USER_MAPPER` | `None` | Puntnotatie‑pad naar `callable(user) -> dict`; hoogste prioriteit. |
| `WIDGET_DEFAULTS` | `{}` | Configuratie samengevoegd in elke widget (camelCase‑sleutels). |
---