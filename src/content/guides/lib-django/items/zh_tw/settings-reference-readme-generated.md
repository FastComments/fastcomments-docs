| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | 您的 FastComments 租戶 ID（測試用的 `demo`）。 |
| `API_KEY` | `""` | 您的 API 密鑰。用於簽署安全 SSO 並驗證 `admin()`。 |
| `REGION` | `None` | 美國使用 `None`，歐盟區域使用 `"eu"`。 |
| `SSO.ENABLED` | `False` | 啟用 SSO。 |
| `SSO.MODE` | `"secure"` | `"secure"`（HMAC）或 `"simple"`（未簽名）。 |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | 顯示給已登出訪客；預設為 `reverse("login"/"logout")`。 |
| `SSO.USER_MAP` | id/email/username | FastComments 欄位對應到使用者屬性/路徑/可呼叫對象。 |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` 或點式路徑。 |
| `SSO.USER_MAPPER` | `None` | 點式路徑指向 `callable(user) -> dict`；具有最高優先權。 |
| `WIDGET_DEFAULTS` | `{}` | 設定會合併到每個小部件（使用 camelCase 鍵）。 |