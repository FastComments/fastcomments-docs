Par défaut FastComments n'autorise pas les iframes dans les commentaires. Lorsque vous activez les intégrations de médias, les commentateurs peuvent coller le code d'intégration (le `<iframe>` snippet) provenant de fournisseurs de confiance comme YouTube, Vimeo, SoundCloud et Spotify, et il s'affichera en ligne dans le commentaire.

Pour des raisons de sécurité, il ne s'agit pas d'un drapeau de configuration du widget côté client. C'est un réglage côté serveur, validé lorsque chaque commentaire est enregistré, donc il ne peut pas être activé depuis la page. Seules les iframes pointant vers une liste intégrée de fournisseurs de confiance sont autorisées. Toute autre iframe est supprimée.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Ajouter vos propres fournisseurs

Si vous souhaitez autoriser les intégrations provenant d'un fournisseur qui ne figure pas dans la liste de confiance intégrée, ajoutez son nom d'hôte dans le champ "Domaines d'intégration supplémentaires" sur la même page. Ces noms d'hôte sont autorisés en plus des fournisseurs intégrés. La correspondance est exacte, donc incluez le nom d'hôte complet (par exemple, player.example.com). Tout ce que vous ne listez pas reste bloqué.

La zone de commentaire classique et l'éditeur WYSIWYG prennent en charge le collage d'un contenu intégré. Dans l'éditeur WYSIWYG, le contenu intégré est inséré comme un bloc supprimable.