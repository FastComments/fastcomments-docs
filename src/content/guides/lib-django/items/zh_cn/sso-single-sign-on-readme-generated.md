Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # 您的 API 密钥；用于签署 Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # 将 FastComments 字段映射到您的用户模型。值可以是属性
        # 名称、点路径 ("profile.avatar_url")、callable(user) 或 None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool，或点路径
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list，或点路径
    },
}
```

> **谨慎选择 SSO `id`。** FastComments 的 `id` 是用户评论历史的永久标识。默认的 `USER_MAP` 将其映射到您的 Django 主键，以实现零配置的便利，但顺序整数主键是可枚举的，且以后难以更改（更改用户的 `id` 会把他们的历史拆分到新账号）。对于任何超出演示的情况，请将 `id` 映射到预先选择的稳定、不可公开的值（如 UUID 或专用公共 id），且永远不要在其中放置私有数据。示例应用程序因此使用基于用户名的 id。

SSO 会自动注入到当前用户的 `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` 和 `{% fastcomments_user_activity %}` 中。

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### 自定义映射

Two higher-precedence options beat `USER_MAP`:

- **对用户模型的一个方法** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **全局映射器**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.