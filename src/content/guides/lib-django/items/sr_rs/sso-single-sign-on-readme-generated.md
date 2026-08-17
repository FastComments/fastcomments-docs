Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # ваш API тајн; потписује Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Мапира FastComments поља на ваш кориснички модел. Вредности могу бити атрибут
        # име, путања са тачкама ("profile.avatar_url"), позив (callable(user)), или None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # позив (user) -> bool, или путања са тачкама
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # позив (user) -> list, или путања са тачкама
    },
}
```

> **Изаберите SSO `id` свесно.** FastComments `id` је трајни идентификатор за историју коментара корисника. Подразумевани `USER_MAP` мапира га на ваш Django примарни кључ за зручност без подешавања, али секвентни целобројни PK‑ови су набројиви и тешко их је касније променити (промена `id` корисника дели њихову историју у нови налог). За све осим демо примера, мапирајте `id` на стабилну, непрозирну вредност изабрану унапред (UUID или посебан јавни id), и никада не стављајте приватне податке у њега. Пример апликације користи id заснован на корисничком имену из овог разлога.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Прилагођено мапирање

Two higher-precedence options beat `USER_MAP`:

- **Метод на вашем корисничком моделу** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобални мапер**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.