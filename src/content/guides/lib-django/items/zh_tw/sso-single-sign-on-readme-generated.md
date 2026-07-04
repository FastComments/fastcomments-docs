Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # 您的 API 密鑰；簽署 Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # 將 FastComments 欄位對映到您的使用者模型。值可以是屬性
        # 名稱、點記法路徑 ("profile.avatar_url")、callable(user) 或 None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool，或點記法路徑
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list，或點記法路徑
    },
}
```

> **請謹慎選擇 SSO `id`。** FastComments 的 `id` 是使用者評論歷史的永久標識。預設的 `USER_MAP` 會將它映射到您的 Django 主鍵，以提供零設定的便利，但順序整數主鍵是可枚舉的，且日後很難更改（更改使用者的 `id` 會將其歷史分割到新帳號）。對於任何非示範用途，請將 `id` 映射為事先選定的穩定且不透明的值（如 UUID 或專用的公開 id），且絕不要在其中放入私人資料。範例應用程式因此使用基於使用者名稱的 id。

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### 自訂映射

Two higher-precedence options beat `USER_MAP`:

- **您使用者模型上的方法** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **全域映射器**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.