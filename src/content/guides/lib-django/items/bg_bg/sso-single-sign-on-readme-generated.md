Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # вашият API секрет; подписва Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Картографирайте полетата на FastComments към вашия модел на потребител. Стойностите могат да бъдат атрибут
        # име, пунктирен път ("profile.avatar_url"), callable(user) или None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, или пунктирен път
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, или пунктирен път
    },
}
```

> **Изберете SSO `id` умишлено.** FastComments `id` е постоянен  
> идентификатор за историята на коментарите на потребителя. По подразбиране `USER_MAP` го съпоставя с вашия  
> Django primary key за удобство без конфигурация, но последователните целочислени PK‑ове са  
> изброими и трудно се променят по-късно (промяната на `id` на потребител разделя тяхната  
> история в нов акаунт). За всичко, което надхвърля демонстрация, съпоставете `id` със стабилна,  
> непрозрачна стойност, избрана предварително (UUID или отделен публичен идентификатор), и никога не поставяйте  
> лични данни в него. Примерното приложение използва `id`, базиран на потребителско име, по тази причина.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and `{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` / `reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Персонализирано съпоставяне

Two higher-precedence options beat `USER_MAP`:

- **Метод във вашия потребителски модел** (питоничният аналог на интерфейс):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобален мапер**, пунктирен път до `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.