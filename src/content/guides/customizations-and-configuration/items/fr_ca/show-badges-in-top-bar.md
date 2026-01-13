[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera les badges des utilisateurs uniquement sur leurs commentaires dans le fil de commentaires.

Cependant, nous pouvons afficher les badges des utilisateurs à côté de leur nom au-dessus du formulaire de commentaire en activant cette fonctionnalité dans la page de personnalisation du widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Cela affichera les badges de l'utilisateur à côté de son nom dans la zone de la barre supérieure, rendant ses réalisations et son statut plus visibles lorsqu'il rédige un commentaire.

Notez que cette fonctionnalité doit être activée dans l'interface de personnalisation du widget pour fonctionner. Vous pouvez éventuellement définir le drapeau **showBadgesInTopBar** sur false dans la configuration de votre code pour la désactiver sélectivement même lorsqu'elle est activée au niveau du serveur:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---