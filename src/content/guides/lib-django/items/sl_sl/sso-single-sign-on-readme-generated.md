Omogočite SSO in izberite način v `settings.py`. Varni SSO podpiše uporabnika
na strani strežnika s HMAC-SHA256 z vašim API skrivnim ključem in je priporočen.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # vaš API skrivni ključ; podpisuje Varni SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Preslikajte polja FastComments na svoj uporabniški model. Vrednosti so lahko
        # ime atributa, pot s pikami ("profile.avatar_url"), callable(user) ali None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ali pot s pikami
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ali pot s pikami
    },
}
```

> **Premislite o izbiri SSO `id`.** FastComments `id` je trajna
> referenca za zgodovino komentarjev uporabnika. Privzeti `USER_MAP` ga preslika na vaš
> Django primarni ključ za udobje brez nastavitev, a zaporedne celoštevilčne PK so
> preštevljivi in jih je kasneje težko spremeniti (sprememba `id` uporabnika razdeli
> njegovo zgodovino v nov račun). Za karkoli več od demo aplikacije preslikajte `id`
> na stabilno, neopisljivo vrednost, izbrano vnaprej (UUID ali namenski javni id), in
> vanjo nikoli ne vključite zasebnih podatkov. Primer aplikacije iz tega razloga
> uporablja id na osnovi uporabniškega imena.

SSO se samodejno vstavi v `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` in
`{% fastcomments_user_activity %}` za trenutnega uporabnika.

URL-ja za prijavo/odjavo, prikazana odjavljenim obiskovalcem, privzeto uporabljata `reverse("login")` /
`reverse("logout")`; preglasita ju z `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Preslikava po meri

Dve možnosti z višjo prednostjo imata prednost pred `USER_MAP`:

- **Metoda na vašem uporabniškem modelu** (pythonična analogija vmesnika):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalni preslikovalnik**, pot s pikami do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prednost ima `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.