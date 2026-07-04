Activez SSO et choisissez un mode dans `settings.py`. Le SSO sécurisé signe l'utilisateur  
côté serveur avec HMAC‑SHA256 en utilisant votre secret d'API et est recommandé.

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

> **Choisissez délibérément le `id` SSO.** Le `id` FastComments est le handle permanent de l'historique des commentaires d'un utilisateur. Le `USER_MAP` par défaut le mappe à la clé primaire de Django pour une commodité sans configuration, mais les PK entiers séquentiels sont énumérables et difficiles à changer plus tard (modifier le `id` d'un utilisateur divise son historique en un nouveau compte). Pour tout usage au‑delà d'une démonstration, mappez le `id` à une valeur stable et opaque choisie à l'avance (un UUID ou un identifiant public dédié), et ne placez jamais de données privées dedans. L'application d'exemple utilise un `id` basé sur le nom d'utilisateur pour cette raison.

Le SSO est injecté automatiquement dans `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` et `{% fastcomments_user_activity %}` pour l'utilisateur actuel.

Les URL de connexion/déconnexion affichées aux visiteurs non connectés utilisent par défaut `reverse("login")` / `reverse("logout")` ; remplacez‑les avec `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Mappage personnalisé

Deux options à priorité supérieure supplantent `USER_MAP` :

- **Une méthode sur votre modèle d'utilisateur** (l'analogue Pythonique d'une interface) :

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Un mapper global**, un chemin pointé vers `callable(user) -> dict` :

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

La priorité est `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.

---