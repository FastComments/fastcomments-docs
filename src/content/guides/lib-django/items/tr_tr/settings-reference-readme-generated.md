| Anahtar | Varsayılan | Açıklama |
|---|---|---|
| `TENANT_ID` | `""` | FastComments kiracı kimliğiniz (`demo` test için). |
| `API_KEY` | `""` | API gizli anahtarınız. Güvenli SSO'yu imzalar ve `admin()`'ı kimlik doğrular. |
| `REGION` | `None` | ABD için `None`, AB bölgesi için `"eu"`. |
| `SSO.ENABLED` | `False` | SSO'yu aç. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) veya `"simple"` (imzasız). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Oturumu kapatmış ziyaretçilere gösterilir; varsayılan `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | FastComments alanı, kullanıcı özniteliği/yolu/çağrılabilir fonksiyona. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` veya noktalı yol. |
| `SSO.USER_MAPPER` | `None` | `callable(user) -> dict` için noktalı yol; en yüksek öncelik. |
| `WIDGET_DEFAULTS` | `{}` | Her widget içine birleştirilen yapılandırma (camelCase anahtarlar). |