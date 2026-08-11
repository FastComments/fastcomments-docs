[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Par défaut, les réponses aux commentaires de niveau supérieur s'affichent.

Cela peut être configuré de façon à ce que l'utilisateur doive cliquer sur « Show Replies » sur les commentaires de niveau supérieur pour voir les réponses.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Option de réduction des réponses dans l\'interface de personnalisation du widget, masquant les commentaires enfants derrière un lien « Afficher les réponses »'; title='Réduire les réponses' app-screenshot-end]

Ce paramètre n'affectera pas le nombre de commentaires de niveau supérieur chargés initialement. Si vous avez un commentaire de niveau supérieur et 29 réponses, avec ce paramètre activé, vous verrez :
- Voir le commentaire de niveau supérieur.
- Voir « Show Replies » (29) sous ce commentaire.

Si vous souhaitez afficher tous les commentaires de niveau supérieur en combinaison avec cette option, définissez [page de démarrage à -1](#starting-page).

---