[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Le nombre de commentaires affiché en haut du widget de commentaires peut être personnalisé.

Cela peut être remplacé par n'importe quelle chaîne, et la valeur **[count]** sera remplacée par le nombre de commentaires, localisé pour l'utilisateur.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Personnalisation du texte du nombre de commentaires'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Champ de texte du nombre de commentaires sur la page de personnalisation du widget, où [count] est remplacé par le total en direct'; title='Personnalisation du texte du nombre de commentaires' app-screenshot-end]