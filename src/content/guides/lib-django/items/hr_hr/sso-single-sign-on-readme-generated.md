Omogućite SSO i odaberite način u `settings.py`. Secure SSO potpisuje korisnika na serverskoj strani koristeći HMAC‑SHA256 s vašim API tajnim ključem i preporučuje se.

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

> **Odaberite SSO `id` svjesno.** FastComments `id` je stalni  
> upravljač za povijest komentara korisnika. Zadani `USER_MAP` ga mapira na vaš  
> Django primarni ključ radi nulte konfiguracije, ali sekvencijalni cijeli PK‑ovi su  
> enumerabilni i teško ih je kasnije promijeniti (promjena `id` korisnika razdvaja  
> njihovu povijest u novi račun). Za sve izvan demonstracije, mapirajte `id` na stabilnu,  
> neprozirnu vrijednost odabranu unaprijed (UUID ili posvećeni javni id), i nikada ne  
> stavljajte privatne podatke u njega. Primjer aplikacije koristi id temeljen na  
> korisničkom imenu iz tog razloga.

SSO se automatski umetne u `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, i `{% fastcomments_user_activity %}` za trenutnog korisnika.

URL‑ovi za prijavu/odjavu prikazani neprijavljenim posjetiteljima zadano su `reverse("login")` / `reverse("logout")`; prepišite ih s `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Custom mapping

Dvije opcije višeg prioriteta imaju prednost nad `USER_MAP`:

- **Metoda na vašem korisničkom modelu** (Python analog sučelja):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapper**, točkasti put do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.