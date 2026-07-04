| Chave | Padrão | Descrição |
|---|---|---|
| `TENANT_ID` | `""` | Seu ID de locatário FastComments (`demo` para teste). |
| `API_KEY` | `""` | Seu segredo da API. Assina SSO Seguro e autentica `admin()`. |
| `REGION` | `None` | `None` para EUA, `"eu"` para a região da UE. |
| `SSO.ENABLED` | `False` | Ativa o SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ou `"simple"` (não assinado). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Exibido para visitantes desconectados; padrão para `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Campo FastComments para atributo/caminho/chamável do usuário. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ou caminho pontilhado. |
| `SSO.USER_MAPPER` | `None` | Caminho pontilhado para `callable(user) -> dict`; precedência mais alta. |
| `WIDGET_DEFAULTS` | `{}` | Configuração mesclada em cada widget (chaves camelCase). |