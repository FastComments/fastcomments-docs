Увімкніть SSO та оберіть режим у `settings.py`. Безпечний SSO підписує користувача
на стороні сервера за допомогою HMAC-SHA256 з використанням вашого API-секрету і є рекомендованим.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # ваш API-секрет; підписує Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Зіставте поля FastComments з вашою моделлю користувача. Значенням може бути
        # ім'я атрибута, шлях через крапку ("profile.avatar_url"), callable(user) або None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, або шлях через крапку
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, або шлях через крапку
    },
}
```

> **Обирайте SSO `id` обдумано.** `id` у FastComments — це постійний
> ідентифікатор історії коментарів користувача. Типовий `USER_MAP` зіставляє його з
> первинним ключем Django для зручності без налаштувань, але послідовні цілочисельні PK
> можна перебирати, і їх важко змінити пізніше (зміна `id` користувача
> розділяє його історію на новий обліковий запис). Для чогось більшого, ніж демо, зіставте `id`
> зі стабільним, непрозорим значенням, обраним заздалегідь (UUID або окремий публічний id),
> і ніколи не вкладайте в нього приватні дані. Приклад застосунку використовує id на основі
> імені користувача саме з цієї причини.

SSO автоматично впроваджується у `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` та
`{% fastcomments_user_activity %}` для поточного користувача.

URL-адреси входу/виходу, що показуються відвідувачам, які не увійшли, за замовчуванням — `reverse("login")` /
`reverse("logout")`; перевизначте їх через `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Користувацьке зіставлення

Два параметри з вищим пріоритетом мають перевагу над `USER_MAP`:

- **Метод у вашій моделі користувача** (пітонічний аналог інтерфейсу):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Глобальний мапер**, шлях через крапку до `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Пріоритет: `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.