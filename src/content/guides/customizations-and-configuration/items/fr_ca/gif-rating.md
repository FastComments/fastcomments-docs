---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Par défaut, le widget de commentaires FastComments attribuera un `gif rating` de `pg`.

Les options disponibles sont `g`, `pg`, `pg-13`, et `r`.

Cela peut être défini dans le code ou via l'interface utilisateur. Dans le code, nous pouvons le faire comme suit:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Définir la cote des Gif'; code-example-end]

Dans l'interface, vous trouverez ceci sous `Gif Picker Rating` tant que `Disable Image Uploads?` n'est pas coché.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Configuration de la cote des Gif' app-screenshot-end]

---