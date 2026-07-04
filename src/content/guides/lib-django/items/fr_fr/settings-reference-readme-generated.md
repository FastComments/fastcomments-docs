| Clé | Valeur par défaut | Description |
|---|---|---|
| `TENANT_ID` | `""` | Votre ID de locataire FastComments (`demo` pour les tests). |
| `API_KEY` | `""` | Votre secret API. Signe le SSO sécurisé et authentifie `admin()`. |
| `REGION` | `None` | `None` pour les États‑Unis, `"eu"` pour la région UE. |
| `SSO.ENABLED` | `False` | Active le SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ou `"simple"` (non signé). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Affiché aux visiteurs déconnectés ; par défaut à `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Champ FastComments vers l’attribut/chemin/appelable de l’utilisateur. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ou chemin pointé. |
| `SSO.USER_MAPPER` | `None` | Chemin pointé vers `callable(user) -> dict` ; priorité la plus élevée. |
| `WIDGET_DEFAULTS` | `{}` | Configuration fusionnée dans chaque widget (clés en camelCase). |