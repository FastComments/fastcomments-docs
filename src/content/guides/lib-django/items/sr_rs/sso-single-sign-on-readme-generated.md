Omogućite SSO i izaberite režim u `settings.py`. Secure SSO potpisuje korisnika na serveru koristeći HMAC‑SHA256 sa vašim API tajnim ključem i preporučuje se.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # vaš API tajni ključ; potpisuje Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapirajte FastComments polja na vaš korisnički model. Vrednosti mogu biti atribut
        # ime, putu razdvojenom tačkama ("profile.avatar_url"), poziv(user), ili None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # poziv(user) -> bool, ili put razdvojen tačkama
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # poziv(user) -> list, ili put razdvojen tačkama
    },
}
```

> **Izaberite SSO `id` namerno.** FastComments `id` je trajni
> identifikator za istoriju komentara korisnika. Podrazumevani `USER_MAP` ga mapira na vaš
> Django primarni ključ radi praktičnosti bez dodatne konfiguracije, ali sekvencijalni integer PK‑ovi su
> brojivi i teško ih je menjati kasnije (promena `id` korisnika deli njihovu
> istoriju u novi nalog). Za sve što prevazilazi demo, mapirajte `id` na stabilnu,
> neprozirnu vrednost izabranu unapred (UUID ili posvećeni javni id), i nikada ne stavljajte
> privatne podatke u njega. Primer aplikacije koristi id zasnovan na korisničkom imenu iz ovog razloga.

SSO se automatski ubacuje u `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i `{% fastcomments_user_activity %}` za trenutnog korisnika.

URL‑ovi za prijavu/odjavu koji se prikazuju posetiocima koji nisu prijavljeni podrazumevano su `reverse("login")` / `reverse("logout")`; možete ih prepisati koristeći `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagođeno mapiranje

Postoje dve opcije višeg prioriteta koje nadmašuju `USER_MAP`:

- **Metod na vašem korisničkom modelu** (Python analog interfejsa):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni mapirator**, put razdvojen tačkama do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioritet je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.