Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC‑SHA256 using your API secret and is recommended.

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

> **Choisissez délibérément le `id` SSO.** Le `id` FastComments est le pointeur permanent de l'historique des commentaires d'un utilisateur. Le `USER_MAP` par défaut le mappe à votre clé primaire Django pour une commodité sans configuration, mais les PK entiers séquentiels sont énumérables et difficiles à modifier plus tard (modifier le `id` d'un utilisateur divise son historique en un nouveau compte). Pour tout ce qui dépasse une démo, mappez le `id` à une valeur stable et opaque choisie à l'avance (un UUID ou un identifiant public dédié), et ne mettez jamais de données privées dedans. L'application d'exemple utilise un `id` basé sur le nom d'utilisateur pour cette raison.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Mappage personnalisé

Two higher-precedence options beat `USER_MAP`:

- **A method on your user model** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **A global mapper**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.