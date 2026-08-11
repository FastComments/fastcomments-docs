[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Par défaut, le widget de commentaires FastComments définira une `gif rating` de `pg`.

Les options disponibles sont `g`, `pg`, `pg-13` et `r`.

Cela peut être défini dans le code ou via l’interface utilisateur. Dans le code, nous pouvons le faire comme suit :

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Définir la note Gif'; code-example-end]

Dans l’interface utilisateur, vous trouverez cela sous `Gif Picker Rating` tant que `Disable Image Uploads?` n’est pas coché.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Liste déroulante Gif Picker Rating sur la page de personnalisation du widget offrant g, pg, pg-13 et r'; title='Définir la note Gif' app-screenshot-end]