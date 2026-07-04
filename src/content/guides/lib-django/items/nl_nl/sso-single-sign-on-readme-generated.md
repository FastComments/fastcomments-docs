Enable SSO en kies een modus in `settings.py`. Secure SSO ondertekent de gebruiker server‑side met HMAC‑SHA256 met behulp van je API‑secret en wordt aanbevolen.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # jouw API‑secret; ondertekent Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Koppel FastComments‑velden aan je gebruikersmodel. Waarden kunnen een attribuut
        # naam, een dotted path ("profile.avatar_url"), een callable(user), of None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, of een dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, of een dotted path
    },
}
```

> **Kies de SSO `id` zorgvuldig.** Het FastComments `id` is de permanente
> identifier voor de reactiegeschiedenis van een gebruiker. De standaard `USER_MAP` koppelt deze aan je
> Django‑primaire‑sleutel voor nul‑configuratiegemak, maar opeenvolgende gehele PK's zijn
> telbaar en moeilijk later te wijzigen (het wijzigen van een gebruikers `id` splitst hun
> geschiedenis over een nieuw account). Voor alles buiten een demo, koppel `id` aan een stabiele,
> ondoorzichtige waarde die vooraf wordt gekozen (een UUID of een toegewijde publieke id), en plaats daar nooit privé‑data in. De voorbeeld‑app gebruikt een op gebruikersnaam gebaseerd id om deze reden.

SSO wordt automatisch geïnjecteerd in `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` en
`{% fastcomments_user_activity %}` voor de huidige gebruiker.

Login-/logout‑URL's die aan uitgelogde bezoekers worden getoond, wijzen standaard naar `reverse("login")` /
`reverse("logout")`; overschrijf ze met `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Custom mapping

Two higher-precedence options beat `USER_MAP`:

- **Een methode op je gebruikersmodel** (de Python‑achtige analog van een interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Een globale mapper**, een dotted path naar `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Prioriteit is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.