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

> **Izaberite SSO `id` namjerno.** FastComments `id` je trajni identifikator za historiju komentara korisnika. Zadani `USER_MAP` mapira ga na vaš primarni ključ iz Django‑a za zero‑config praktičnost, ali sekvencijalni integer PK‑ovi su enumerabilni i teško je mijenjati kasnije (promjena `id`‑a korisnika razdvaja njihovu historiju u novi račun). Za sve osim demonstracije, mapirajte `id` na stabilnu, neprozirnu vrijednost odabranu unaprijed (UUID ili posvećeni javni id), i nikada ne stavljajte privatne podatke u njega. Primjer aplikacije koristi id baziran na korisničkom imenu iz ovog razloga.

SSO se automatski ubacuje u `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i `{% fastcomments_user_activity %}` za trenutnog korisnika.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` / `reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagođeno mapiranje

Dve opcije višeg prioriteta nadjačavaju `USER_MAP`:

- **Metod na vašem modelu korisnika** (Python‑ski analog interfejsa):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapper**, putanja odvojena tačkama do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.