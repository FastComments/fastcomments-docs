在 `settings.py` 中启用 SSO 并选择一种模式。安全 SSO 使用你的 API 密钥在服务端通过 HMAC-SHA256 对用户进行签名，推荐使用。

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # 你的 API 密钥；用于对安全 SSO 进行签名
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # 将 FastComments 字段映射到你的用户模型。值可以是属性名、点分路径
        # （"profile.avatar_url"）、可调用对象 callable(user)，或 None。
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool，或点分路径
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list，或点分路径
    },
}
```

> **慎重选择 SSO 的 `id`。** FastComments 的 `id` 是用户评论历史的永久标识。默认的 `USER_MAP` 将其映射到你的 Django 主键以实现零配置便利，但顺序整数主键是可枚举的，且之后很难更改（更改用户的 `id` 会将其历史拆分到一个新账户）。除了演示之外，请将 `id` 映射到一个预先选定的稳定、不透明的值（UUID 或专用的公开 id），并且绝不要在其中放入私有数据。示例应用正是出于这个原因使用了基于用户名的 id。

SSO 会自动注入到 `{% fastcomments %}`、`{% fastcomments_live_chat %}`、`{% fastcomments_collab_chat %}`、`{% fastcomments_image_chat %}` 和 `{% fastcomments_user_activity %}` 中，供当前用户使用。

向未登录访客显示的登录/登出 URL 默认为 `reverse("login")` / `reverse("logout")`；可以通过 `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]` 覆盖它们。

### 自定义映射

两个更高优先级的选项会覆盖 `USER_MAP`：

- **用户模型上的方法**（接口的 Python 风格等价物）：

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **全局映射器**，一个指向 `callable(user) -> dict` 的点分路径：

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

优先级为 `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`。