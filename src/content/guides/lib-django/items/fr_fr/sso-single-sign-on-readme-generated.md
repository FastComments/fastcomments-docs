Activez le SSO et choisissez un mode dans `settings.py`. Le SSO sécurisé signe l'utilisateur
côté serveur avec HMAC-SHA256 en utilisant votre secret d'API et est recommandé.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # votre secret d'API ; signe le SSO sécurisé
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mappez les champs FastComments à votre modèle utilisateur. Les valeurs peuvent être
        # un nom d'attribut, un chemin pointé ("profile.avatar_url"), un callable(user), ou None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ou chemin pointé
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ou chemin pointé
    },
}
```

> **Choisissez délibérément le `id` du SSO.** Le `id` FastComments est l'identifiant
> permanent de l'historique de commentaires d'un utilisateur. Le `USER_MAP` par défaut le mappe
> à votre clé primaire Django pour plus de commodité sans configuration, mais les clés primaires entières
> séquentielles sont énumérables et difficiles à changer plus tard (changer le `id` d'un utilisateur
> divise son historique en un nouveau compte). Pour tout usage au-delà d'une démo, mappez `id` à une
> valeur stable et opaque choisie dès le départ (un UUID ou un identifiant public dédié), et
> n'y mettez jamais de données privées. L'application d'exemple utilise un identifiant basé sur le
> nom d'utilisateur pour cette raison.

Le SSO est injecté automatiquement dans `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` et
`{% fastcomments_user_activity %}` pour l'utilisateur courant.

Les URLs de connexion/déconnexion affichées aux visiteurs non connectés pointent par défaut vers
`reverse("login")` / `reverse("logout")` ; remplacez-les avec `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Mapping personnalisé

Deux options de priorité supérieure priment sur `USER_MAP` :

- **Une méthode sur votre modèle utilisateur** (l'équivalent Pythonique d'une interface) :

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Un mappeur global**, un chemin pointé vers `callable(user) -> dict` :

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

La priorité est `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.