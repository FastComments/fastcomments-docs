---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera les badges des utilisateurs uniquement sur leurs commentaires au sein du fil de discussion.

Cependant, nous pouvons afficher les badges des utilisateurs à côté de leur nom au-dessus du formulaire de commentaire en activant cette fonctionnalité sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Case à cocher Afficher les badges dans la barre supérieure sur la page de personnalisation du widget, plaçant les badges à côté du nom au-dessus du formulaire de commentaire'; title='Option Afficher les badges dans la barre supérieure' app-screenshot-end]

Cela affichera les badges de l'utilisateur à côté de son nom dans la zone de la barre supérieure, rendant leurs réalisations et leur statut plus visibles lorsqu'ils rédigent un commentaire.

Notez que cette fonctionnalité doit être activée dans l'interface de personnalisation du widget pour fonctionner. Vous pouvez éventuellement définir le drapeau **showBadgesInTopBar** sur false dans votre configuration de code afin de le désactiver sélectivement même lorsqu'il est activé au niveau du serveur :

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---