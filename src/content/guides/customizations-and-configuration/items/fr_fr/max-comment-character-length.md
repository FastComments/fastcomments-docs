[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Le nombre maximal de caractères autorisés à être saisis dans le champ de saisie du commentaire peut être limité par le paramètre **maxCommentCharacterLength**.

La valeur par défaut est 2000.

Des éléments tels que les URL d'images ne sont pas pris en compte dans le calcul de la longueur.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Champ de taille maximale du commentaire sur la page de personnalisation du widget, utilisé pour limiter le nombre de caractères qu\'un commentaire peut contenir'; title='Limiter la longueur du commentaire' app-screenshot-end]