| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Twój identyfikator najemcy FastComments (`demo` do testów). |
| `API_KEY` | `""` | Twój tajny klucz API. Podpisuje Secure SSO i uwierzytelnia `admin()`. |
| `REGION` | `None` | `None` dla USA, `"eu"` dla regionu UE. |
| `SSO.ENABLED` | `False` | Włącz SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) lub `"simple"` (niepodpisane). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Wyświetlane niezalogowanym odwiedzającym; domyślnie `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Pole FastComments wskazujące na atrybut/ścieżkę/wywołanie zwrotne użytkownika. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` lub ścieżka kropkowa. |
| `SSO.USER_MAPPER` | `None` | Ścieżka kropkowa do `callable(user) -> dict`; najwyższy priorytet. |
| `WIDGET_DEFAULTS` | `{}` | Konfiguracja łączona z każdym widżetem (klucze w stylu camelCase). |