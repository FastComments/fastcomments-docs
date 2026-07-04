Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # ваш секрет API; подписывает Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Прив'язка полів FastComments до вашої моделі користувача. Значення можуть бути атрибутом
        # назвою, шляхом з крапками ("profile.avatar_url"), callable(user) або None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, або шлях з крапками
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, або шлях з крапками
    },
}
```

> **Выбирайте SSO `id` осознанно.** FastComments `id` — это постоянный идентификатор истории комментариев пользователя. По умолчанию `USER_MAP` сопоставляет его с вашим первичным ключом Django для удобства без настройки, но последовательные целочисленные PK легко перечислить и позднее изменить (изменение `id` пользователя разбивает его историю на новый аккаунт). Для любого использования, кроме демо, сопоставьте `id` со стабильным, непрозрачным значением, выбранным заранее (UUID или отдельным публичным идентификатором), и никогда не помещайте в него личные данные. Пример приложения использует идентификатор на основе имени пользователя по этой причине.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Пользовательское сопоставление

Two higher-precedence options beat `USER_MAP`:

- **A method on your user model** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **A global mapper**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.