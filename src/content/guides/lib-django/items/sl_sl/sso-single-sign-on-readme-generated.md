Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC‑SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # vaš API skrivni ključ; podpira Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Preslikajte polja FastComments v vaš uporabniški model. Vrednosti so lahko atribut
        # ime, pot v obliki točk ("profile.avatar_url"), klicna funkcija(user) ali None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # klicna funkcija(user) -> bool, ali pot v obliki točk
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # klicna funkcija(user) -> list, ali pot v obliki točk
    },
}
```

> **Izberite SSO `id` namerno.** FastComments `id` je trajen
> identifikator za zgodovino komentarjev uporabnika. Privzeti `USER_MAP` ga preslika v vaš
> Django primarni ključ za priročnost brez dodatne konfiguracije, vendar so zaporedni celoštevilski PK-ji
> preštevljivi in jih je kasneje težko spremeniti (sprememba uporabnikovega `id` razdeli njihovo
> zgodovino v nov račun). Za vse, kar presega demo, preslikajte `id` v stabilno,
> neprozorno vrednost, izbrano vnaprej (UUID ali namenski javni id), in v njem nikoli ne shranjujte
> zasebnih podatkov. Primerna aplikacija uporablja za to namen ID, ki temelji na uporabniškem imenu.

SSO se vključi samodejno v `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` in
`{% fastcomments_user_activity %}` za trenutnega uporabnika.

Login/logout URLs shown to signed‑out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Prilagojeno preslikovanje

Dve možnosti z višjo prednostjo presegata `USER_MAP`:

- **Metoda v vašem uporabniškem modelu** (Pythonic analog vmesnika):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni preslikovalec**, pot v obliki točk do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prednost je `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.