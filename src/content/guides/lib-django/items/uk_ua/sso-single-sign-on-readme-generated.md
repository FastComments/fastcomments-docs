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

> **Вибирайте SSO `id` свідомо.** FastComments `id` — це постійний
> ідентифікатор історії коментарів користувача. За замовчуванням
> `USER_MAP` прив'язує його до вашого первинного ключа Django для
> зручності без налаштувань, але послідовні цілі числові PK є
> перелічуваними та їх важко пізніше змінити (зміна `id` користувача
> розділяє його історію на новий обліковий запис). Для будь‑якого
> використання понад демо, прив'яжіть `id` до стабільного, непрозорого
> значення, вибраного заздалегідь (UUID або присвячений публічний id),
> і ніколи не розміщуйте в ньому приватні дані. Приклад застосунку
> використовує id, оснований на імені користувача, з цієї причини.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Кастомне зіставлення

Two higher-precedence options beat `USER_MAP`:

- **Метод у вашій моделі користувача** (пайтонічний аналог інтерфейсу):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобальний мапер**, крапковий шлях до `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.