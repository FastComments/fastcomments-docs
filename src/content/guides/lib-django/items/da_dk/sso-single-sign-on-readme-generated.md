Enable SSO og vælg en tilstand i `settings.py`. Secure SSO underskriver brugeren på server‑siden med HMAC‑SHA256 ved hjælp af din API‑hemmelighed og anbefales.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # din API‑hemmelighed; underskriver Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Kortlæg FastComments‑felter til din brugermodel. Værdier kan være et attribut
        # navn, en punktnotation‑sti ("profile.avatar_url"), et callable(user), eller None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, eller punktnotation‑sti
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, eller punktnotation‑sti
    },
}
```

> **Vælg SSO `id` bevidst.** FastComments `id` er den permanente
> håndtag for en brugers kommentarthistorik. Standard-`USER_MAP` kortlægger den til din
> Django‑primærnøgle for nul‑konfigurations‑bekvemmelighed, men sekventielle heltals‑PK'er er
> gennemskuelige og svære at ændre senere (ændring af en brugers `id` opdeler deres
> historik i en ny konto). For alt ud over en demo, kortlæg `id` til en stabil,
> uigennemsigtig værdi valgt på forhånd (en UUID eller et dedikeret offentligt id), og læg aldrig
> private data i den. Eksempel‑appen bruger et brugernavns‑baseret id af denne grund.

SSO injiceres automatisk i `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` og `{% fastcomments_user_activity %}` for den aktuelle bruger.

Login-/logout‑URL’er, der vises for ikke‑loggede besøgende, standard er `reverse("login")` / `reverse("logout")`; tilsidesæt dem med `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Tilpasset kortlægning

To højere præcedens‑indstillinger har forrang over `USER_MAP`:

- **En metode på din brugermodel** (den Python‑ækvivalente analog til et interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **En global mapper**, en punktnotation‑sti til `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Præcedens er `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.