Активирайте SSO и изберете режим в `settings.py`. Secure SSO подписва потребителя
от сървърната страна с HMAC-SHA256, използвайки вашия API secret, и е препоръчителният вариант.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # вашият API secret; подписва Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Свържете полетата на FastComments с вашия потребителски модел. Стойностите могат да са
        # име на атрибут, точков път ("profile.avatar_url"), callable(user) или None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, или точков път
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, или точков път
    },
}
```

> **Изберете SSO `id`-то обмислено.** `id`-то в FastComments е постоянният
> идентификатор на историята от коментари на потребителя. Стандартният `USER_MAP` го свързва
> с вашия Django primary key за удобство без конфигурация, но последователните целочислени PK са
> лесни за изброяване и трудни за промяна по-късно (промяната на `id` на потребител разделя
> историята му в нов акаунт). За всичко отвъд демо, свържете `id` със стабилна,
> непрозрачна стойност, избрана предварително (UUID или отделен публичен идентификатор), и никога
> не поставяйте лични данни в него. Примерното приложение използва id, базиран на потребителско име, именно по тази причина.

SSO се инжектира автоматично в `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` и
`{% fastcomments_user_activity %}` за текущия потребител.

URL адресите за login/logout, показвани на посетители, които не са влезли, по подразбиране са `reverse("login")` /
`reverse("logout")`; презапишете ги с `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Персонализирано съпоставяне

Две опции с по-висок приоритет имат предимство пред `USER_MAP`:

- **Метод във вашия потребителски модел** (питоническият аналог на интерфейс):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобален mapper**, точков път до `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Приоритетът е `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.