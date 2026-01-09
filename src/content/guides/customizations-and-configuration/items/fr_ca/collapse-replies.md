[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Par défaut, les réponses aux commentaires de niveau supérieur sont affichées.

Cela peut être configuré afin que l'utilisateur doive cliquer sur "Afficher les réponses" sur les commentaires de niveau supérieur pour voir les réponses enfants.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ce paramètre n'affectera pas le nombre de commentaires de niveau supérieur chargés initialement. Si vous avez un commentaire de niveau supérieur et 29 réponses enfants, avec ce réglage activé vous allez :

- Voir le commentaire de niveau supérieur.
- Voir "Afficher les réponses (29)" sous ce commentaire.

Si vous souhaitez afficher tous les commentaires de niveau supérieur en combinaison avec cette option, définissez [la page de départ sur -1](#starting-page).