Par défaut FastComments n'autorise pas les iframes dans les commentaires. Lorsque vous activez les intégrations multimédias, les commentateurs peuvent coller le code d'intégration (l'extrait `<iframe>`) provenant de fournisseurs de confiance comme YouTube, Vimeo, SoundCloud et Spotify, et il s'affichera directement dans le commentaire.

Pour des raisons de sécurité, il ne s'agit pas d'un paramètre de configuration du widget côté client. C'est un réglage côté serveur, validé à l'enregistrement de chaque commentaire, donc il ne peut pas être activé depuis la page. Seuls les iframes pointant vers une liste intégrée de fournisseurs de confiance sont autorisés. Tout autre iframe est supprimé.

This is done without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Ajouter vos propres fournisseurs

Si vous souhaitez autoriser des intégrations provenant d'un fournisseur qui ne figure pas dans la liste intégrée de confiance, ajoutez son nom d'hôte dans le champ "Domaines d'intégration supplémentaires" de la même page. Ces noms d'hôte sont autorisés en plus des fournisseurs intégrés. La correspondance est exacte, donc incluez le nom d'hôte complet (par exemple, player.example.com). Tout ce que vous ne listez pas reste bloqué.

La zone de commentaire simple et l'éditeur WYSIWYG prennent tous deux en charge le collage d'une intégration. Dans l'éditeur WYSIWYG, l'intégration est insérée comme un bloc amovible.

---