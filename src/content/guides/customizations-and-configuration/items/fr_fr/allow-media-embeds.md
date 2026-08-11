Par défaut, FastComments n’autorise pas les iframes dans les commentaires. Lorsque vous activez les intégrations multimédias, les commentateurs peuvent coller le code d’intégration (l’extrait `<iframe>`) provenant de fournisseurs de confiance tels que YouTube, Vimeo, SoundCloud et Spotify, et il sera affiché en ligne dans le commentaire.

Pour des raisons de sécurité, il ne s’agit pas d’un drapeau de configuration du widget côté client. C’est un paramètre côté serveur, validé lors de l’enregistrement de chaque commentaire, il ne peut donc pas être activé depuis la page. Seules les iframes pointant vers une liste intégrée de fournisseurs de confiance sont autorisées. Toute autre iframe est supprimée.

Cela se fait sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Paramètre d\'intégration multimédia activé sur la page de personnalisation du widget, permettant aux commentateurs de coller des intégrations iframe de confiance'; title='Autoriser les intégrations multimédias' app-screenshot-end]

### Ajouter vos propres fournisseurs

Si vous souhaitez autoriser les intégrations provenant d’un fournisseur qui ne figure pas sur la liste intégrée de fournisseurs de confiance, ajoutez son nom d’hôte dans le champ « Domaines d’intégration supplémentaires » sur la même page. Ces noms d’hôte sont autorisés en plus des fournisseurs intégrés. La correspondance est exacte, donc incluez le nom d’hôte complet (par exemple, player.example.com). Tout ce que vous ne répertoriez pas reste bloqué.

La zone de commentaire simple ainsi que l’éditeur WYSIWYG prennent en charge le collage d’une intégration. Dans l’éditeur WYSIWYG, l’intégration est insérée sous forme de bloc amovible.