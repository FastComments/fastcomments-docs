Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # your API secret; signs Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Map FastComments fields to your user model. Values may be an attribute
        # name, a dotted path ("profile.avatar_url"), a callable(user), or None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, or dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, or dotted path
    },
}
```

> **Выбирайте SSO `id` осознанно.** FastComments `id` — это постоянный
> идентификатор истории комментариев пользователя. По умолчанию `USER_MAP` сопоставляет его с вашей
> первичным ключом Django для удобства без настройки, но последовательные целочисленные PK легко перечислить и сложно изменить позже (изменение `id` пользователя разбивает их
> историю на новую учётную запись). Для любого использования, выходящего за рамки демонстрации, сопоставляйте `id` со стабильным,
> непрозрачным значением, выбранным заранее (UUID или отдельный публичный id), и никогда не помещайте в него
> закрытые данные. Пример приложения использует идентификатор на основе имени пользователя по этой причине.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` and `{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` / `reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Пользовательское сопоставление

Two higher-precedence options beat `USER_MAP`:

- **Метод в вашей модели пользователя** (питонический аналог интерфейса):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобальный маппер**, точечный путь к `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.