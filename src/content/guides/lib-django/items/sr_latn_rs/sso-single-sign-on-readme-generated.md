Omogućite SSO i izaberite režim u `settings.py`. Secure SSO potpisuje korisnika na server‑side koristeći HMAC‑SHA256 i vaš API tajni i preporučuje se.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # vaš API tajni; potpisuje Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapirajte FastComments polja na vaš model korisnika. Vrednosti mogu biti atribut
        # ime, putanja sa tačkama ("profile.avatar_url"), callable(user), ili None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ili putanja sa tačkama
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ili putanja sa tačkama
    },
}
```

> **Pažljivo izaberite SSO `id`.** FastComments `id` je trajni identifikator
> za istoriju komentara korisnika. Podrazumevani `USER_MAP` mapira ga na vaš
> Django primarni ključ radi praktičnosti bez podešavanja, ali sekvencijalni
> integer PK‑ovi su izlistivi i teški za promenu kasnije (promena `id`‑a
> korisnika deli njihovu istoriju u novi nalog). Za sve što nije demo,
> mapirajte `id` na stabilnu, neprozirnu vrednost izabranu unapred (UUID ili
> poseban javni id), i nikada ne stavljajte privatne podatke u njega. Primer
> aplikacije koristi `id` zasnovan na korisničkom imenu iz tog razloga.

SSO se automatski ubacuje u `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i
`{% fastcomments_user_activity %}` za trenutnog korisnika.

URL‑ovi za prijavu/odjavu koji se prikazuju posetiocima bez naloga podrazumevano su `reverse("login")` /
`reverse("logout")`; možete ih zameniti sa `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagođeno mapiranje

Dve opcije većeg prioriteta imaju prednost nad `USER_MAP`:

- **Metoda na vašem modelu korisnika** (Pythonic analog interfejsa):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapper**, putanja sa tačkama do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.