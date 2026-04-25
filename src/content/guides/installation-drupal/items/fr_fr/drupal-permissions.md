Le module ajoute trois autorisations Drupal que vous pouvez attribuer par rôle sous `People > Permissions`.

- **Administer FastComments** - Accès au formulaire de configuration de FastComments à `/admin/config/content/fastcomments`.
- **View FastComments** - Requise pour voir le widget de commentaires. Sans cette autorisation, le widget ne s'affiche pas.
- **Toggle FastComments** - Permet aux utilisateurs d'activer ou de désactiver les commentaires au niveau de chaque entité en utilisant le widget de champ.

Par défaut, seuls les utilisateurs disposant de l'autorisation `administer site configuration` peuvent modifier les paramètres de FastComments. Accordez `View FastComments` aux utilisateurs anonymes et authentifiés si vous voulez que les visiteurs voient le widget.