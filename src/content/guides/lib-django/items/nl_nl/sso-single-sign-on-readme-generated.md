Schakel SSO in en kies een modus in `settings.py`. Secure SSO ondertekent de gebruiker
server-side met HMAC-SHA256 met behulp van je API secret en wordt aanbevolen.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # je API secret; ondertekent Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Koppel FastComments-velden aan je user model. Waarden kunnen een attribuut-
        # naam zijn, een dotted path ("profile.avatar_url"), een callable(user), of None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, of dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, of dotted path
    },
}
```

> **Kies de SSO `id` bewust.** De FastComments `id` is de permanente
> verwijzing naar de commentaarhistorie van een gebruiker. De standaard `USER_MAP` koppelt
> deze aan je Django primary key voor zero-config gemak, maar sequentiële integer PK's zijn
> raadpleegbaar en moeilijk later te wijzigen (het wijzigen van de `id` van een gebruiker
> splitst hun historie in een nieuw account). Voor alles meer dan een demo, koppel `id` aan
> een stabiele, ondoorzichtige waarde die vooraf gekozen is (een UUID of een speciaal
> publiek id), en plaats nooit privégegevens erin. De voorbeeldapp gebruikt daarom een
> op gebruikersnaam gebaseerd id.

SSO wordt automatisch geïnjecteerd in `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, en
`{% fastcomments_user_activity %}` voor de huidige gebruiker.

Login/logout-URL's die worden getoond aan uitgelogde bezoekers verwijzen standaard naar
`reverse("login")` / `reverse("logout")`; overschrijf deze met
`SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Aangepaste mapping

Twee opties met hogere prioriteit gaan vóór `USER_MAP`:

- **Een methode op je user model** (het Pythonische equivalent van een interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Een globale mapper**, een dotted path naar `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

De prioriteit is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.