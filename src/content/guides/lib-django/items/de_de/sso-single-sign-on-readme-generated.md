Aktiviere SSO und wähle einen Modus in `settings.py`. Secure SSO signiert den Benutzer serverseitig mit HMAC-SHA256 unter Verwendung deines API-Secrets und wird empfohlen.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # dein API-Secret; signiert Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Ordne FastComments-Felder deinem Benutzermodell zu. Werte können ein Attributname,
        # ein gepunkteter Pfad ("profile.avatar_url"), eine callable(user) oder None sein.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, oder gepunkteter Pfad
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, oder gepunkteter Pfad
    },
}
```

> **Wähle die SSO-`id` bewusst.** Die FastComments-`id` ist der permanente
> Handle für den Kommentarverlauf eines Benutzers. Die Standard-`USER_MAP` ordnet sie deinem
> Django-Primärschlüssel zu, um eine Zero-Config-Bequemlichkeit zu bieten, aber sequenzielle Integer-PKs sind
> aufzählbar und später schwer zu ändern (das Ändern der `id` eines Benutzers teilt seinen
> Verlauf in ein neues Konto auf). Für alles über eine Demo hinaus, ordne `id` einem stabilen,
> opaken Wert zu, der von vornherein gewählt wird (eine UUID oder eine dedizierte öffentliche ID), und lege
> niemals private Daten darin ab. Die Beispiel-App verwendet aus diesem Grund eine benutzernamebasierte ID.

SSO wird automatisch in `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` und
`{% fastcomments_user_activity %}` für den aktuellen Benutzer injiziert.

An- und Abmelde-URLs, die abgemeldeten Besuchern angezeigt werden, sind standardmäßig `reverse("login")` /
`reverse("logout")`; überschreibe sie mit `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Benutzerdefiniertes Mapping

Zwei Optionen mit höherer Priorität schlagen `USER_MAP`:

- **Eine Methode auf deinem Benutzermodell** (das Python-Analogon zu einem Interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Ein globaler Mapper**, ein gepunkteter Pfad zu `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Die Priorität ist `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.