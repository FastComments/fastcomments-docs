Включите SSO и выберите режим в `settings.py`. Secure SSO подписывает пользователя
на стороне сервера с помощью HMAC-SHA256, используя ваш API secret, и является рекомендуемым вариантом.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # ваш API secret; подписывает Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Сопоставьте поля FastComments с вашей моделью пользователя. Значением может быть
        # имя атрибута, путь через точку ("profile.avatar_url"), callable(user) или None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, или путь через точку
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, или путь через точку
    },
}
```

> **Осознанно выбирайте SSO `id`.** `id` в FastComments — это постоянный
> идентификатор истории комментариев пользователя. По умолчанию `USER_MAP` сопоставляет его
> с первичным ключом Django для удобства настройки «из коробки», но последовательные
> целочисленные PK перебираемы, и их сложно изменить позже (смена `id` пользователя
> разделяет его историю на новый аккаунт). Для чего-либо помимо демо сопоставьте `id`
> со стабильным, непрозрачным значением, выбранным заранее (UUID или отдельный публичный id),
> и никогда не помещайте в него приватные данные. Пример приложения использует id на основе
> имени пользователя именно по этой причине.

SSO автоматически внедряется в `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` и
`{% fastcomments_user_activity %}` для текущего пользователя.

URL-адреса входа/выхода, отображаемые неавторизованным посетителям, по умолчанию
указывают на `reverse("login")` / `reverse("logout")`; переопределите их с помощью
`SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Пользовательское сопоставление

Два варианта с более высоким приоритетом имеют преимущество над `USER_MAP`:

- **Метод в вашей модели пользователя** (питонический аналог интерфейса):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобальный маппер** — путь через точку к `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Приоритет: `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.