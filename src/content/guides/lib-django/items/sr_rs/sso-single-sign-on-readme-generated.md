Омогућите SSO и изаберите режим у `settings.py`. Secure SSO потписује корисника
серверски помоћу HMAC-SHA256 са вашим API секретом и препоручен је.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # ваш API секрет; потписује Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Мапирајте FastComments поља на ваш модел корисника. Вредности могу бити
        # име атрибута, пут са тачкама ("profile.avatar_url"), callable(user) или None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, или пут са тачкама
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, или пут са тачкама
    },
}
```

> **Свесно изаберите SSO `id`.** FastComments `id` је трајни идентификатор
> историје коментара корисника. Подразумевани `USER_MAP` мапира га на ваш
> Django примарни кључ ради погодности без конфигурације, али секвенцијални
> целобројни PK-јеви су енумерабилни и тешко се мењају касније (промена `id`
> корисника дели његову историју на нови налог). За било шта изван демоа,
> мапирајте `id` на стабилну, непрозрачну вредност изабрану унапред (UUID или
> посебан јавни id) и никада не стављајте приватне податке у њега. Пример
> апликације користи id заснован на корисничком имену управо из тог разлога.

SSO се аутоматски убацива у `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` и
`{% fastcomments_user_activity %}` за текућег корисника.

URL-ови за пријаву/одјаву приказани одјављеним посетиоцима подразумевано су
`reverse("login")` / `reverse("logout")`; преписујте их са
`SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Прилагођено мапирање

Две опције вишег приоритета имају предност над `USER_MAP`:

- **Метод на вашем моделу корисника** (питонски аналог интерфејса):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобални мапер**, пут са тачкама до `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Редослед приоритета је `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.