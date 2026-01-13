---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Le nombre maximal de caractères pouvant être saisis dans le champ de commentaire peut être limité par le paramètre **maxCommentCharacterLength**.

La valeur par défaut est 2000.

Des éléments comme les URL d'images ne sont pas inclus dans le calcul de la longueur.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---