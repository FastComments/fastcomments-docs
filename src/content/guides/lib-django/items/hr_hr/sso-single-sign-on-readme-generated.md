Omogućite SSO i odaberite način rada u `settings.py`. Secure SSO potpisuje korisnika
na strani poslužitelja pomoću HMAC-SHA256 s vašim API tajnim ključem i preporučuje se.

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

> **Svjesno odaberite SSO `id`.** FastComments `id` je trajni identifikator
> povijesti komentara korisnika. Zadani `USER_MAP` preslikava ga na vaš
> Django primarni ključ radi praktičnosti bez konfiguracije, ali sekvencijalni
> cjelobrojni PK-ovi su pogodni za pogađanje i teško ih je kasnije promijeniti
> (promjena `id` korisnika razdvaja njegovu povijest u novi račun). Za bilo što
> iznad demonstracije, preslikajte `id` na stabilnu, neprozirnu vrijednost
> odabranu unaprijed (UUID ili namjenski javni id) i nikada ne stavljajte
> privatne podatke u njega. Primjer aplikacije iz tog razloga koristi id
> temeljen na korisničkom imenu.

SSO se automatski ubacuje u `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i
`{% fastcomments_user_activity %}` za trenutnog korisnika.

URL-ovi za prijavu/odjavu prikazani posjetiteljima koji nisu prijavljeni
zadano su `reverse("login")` / `reverse("logout")`; nadjačajte ih s
`SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagođeno preslikavanje

Dvije opcije višeg prioriteta nadjačavaju `USER_MAP`:

- **Metoda na vašem korisničkom modelu** (Pythonov analog sučelja):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapper**, točkasta putanja do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.