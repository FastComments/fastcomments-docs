在 `settings.py` 中啟用 SSO 並選擇模式。安全 SSO（Secure SSO）會使用您的 API 密鑰在伺服器端以 HMAC-SHA256 簽署使用者，建議使用此模式。

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # 您的 API 密鑰；用於簽署安全 SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # 將 FastComments 欄位對應到您的使用者模型。值可以是屬性名稱、
        # 點號路徑（"profile.avatar_url"）、callable(user)，或 None。
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool，或點號路徑
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list，或點號路徑
    },
}
```

> **請審慎選擇 SSO 的 `id`。** FastComments 的 `id` 是使用者留言歷史的永久識別碼。預設的 `USER_MAP` 為了零設定的便利性，會將它對應到您的 Django 主鍵，但連續整數主鍵容易被列舉，且日後難以更改（更改使用者的 `id` 會將其歷史拆分到一個新帳號）。除了示範之外的任何用途，請將 `id` 對應到預先選定的穩定、不透明值（UUID 或專用的公開 id），並且切勿在其中放入私人資料。範例應用程式基於此原因使用以使用者名稱為基礎的 id。

SSO 會自動注入到目前使用者的 `{% fastcomments %}`、`{% fastcomments_live_chat %}`、`{% fastcomments_collab_chat %}`、`{% fastcomments_image_chat %}` 與 `{% fastcomments_user_activity %}` 中。

顯示給未登入訪客的登入/登出 URL 預設為 `reverse("login")` / `reverse("logout")`；可透過 `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]` 覆寫。

### 自訂對應

兩個優先權更高的選項會覆蓋 `USER_MAP`：

- **使用者模型上的方法**（介面的 Pythonic 對應物）：

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **全域對應器**，一個指向 `callable(user) -> dict` 的點號路徑：

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

優先順序為 `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`。