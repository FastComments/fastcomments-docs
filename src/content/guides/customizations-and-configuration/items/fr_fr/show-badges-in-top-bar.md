[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments n'affiche les badges des utilisateurs que sur leurs commentaires dans le fil de discussion.

Cependant, nous pouvons afficher les badges des utilisateurs à côté de leur nom, au-dessus du formulaire de commentaire, en activant cette fonctionnalité dans la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Cela affichera les badges de l'utilisateur à côté de son nom dans la zone de la barre supérieure, rendant ses réalisations et son statut plus visibles lorsqu'il rédige un commentaire.

Notez que cette fonctionnalité doit être activée dans l'interface de personnalisation du widget pour fonctionner. Vous pouvez éventuellement définir le flag **showBadgesInTopBar** sur false dans votre configuration de code pour la désactiver sélectivement, même si elle est activée côté serveur :

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---