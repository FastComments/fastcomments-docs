Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # tvoj API tajni ključ; potpisuje Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapiraj FastComments polja na svoj korisnički model. Vrijednosti mogu biti atribut
        # ime, dotična putanja ("profile.avatar_url"), callable(user), ili None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ili dotična putanja
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ili dotična putanja
    },
}
```

> **Odaberi SSO `id` svjesno.** FastComments `id` je trajni identifikator za istoriju komentara korisnika. Zadani `USER_MAP` ga mapira na tvoj Django primarni ključ radi zero-config praktičnosti, ali sekvencijalni integer PK-ovi su izlistivi i teško je mijenjati ih kasnije (promjena `id` korisnika dijeli njihovu historiju u novi račun). Za sve što je izvan demo verzije, mapiraj `id` na stabilnu, neprozirnu vrijednost izabranu unaprijed (UUID ili posvećeni javni id), i nikada ne stavljaj privatne podatke u njega. Primjer aplikacije koristi id zasnovan na korisničkom imenu iz tog razloga.

SSO se automatski ubacuje u `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i `{% fastcomments_user_activity %}` za trenutnog korisnika.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagođeno mapiranje

Dve opcije višeg priorитета imaju prednost nad `USER_MAP`:

- **Metod na tvom korisničkom modelu** (Python analog interface-a):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapper**, dotična putanja do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.