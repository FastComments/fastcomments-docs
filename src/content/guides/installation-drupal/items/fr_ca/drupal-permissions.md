---
Le module ajoute trois permissions Drupal que vous pouvez attribuer par rôle sous `People > Permissions`.

- **Administer FastComments** - Accès au formulaire de configuration de FastComments à `/admin/config/content/fastcomments`.
- **View FastComments** - Nécessaire pour voir le widget de commentaires. Sans cette permission, le widget ne s'affiche pas.
- **Toggle FastComments** - Permet aux utilisateurs d'activer ou de désactiver les commentaires pour chaque entité à l'aide du widget de champ.

Par défaut, seuls les utilisateurs disposant de la permission `administer site configuration` peuvent modifier les paramètres de FastComments. Accordez la permission `View FastComments` aux utilisateurs anonymes et authentifiés si vous voulez que les visiteurs voient le widget.

---