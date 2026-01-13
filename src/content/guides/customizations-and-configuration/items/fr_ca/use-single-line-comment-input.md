[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet à l'utilisateur de saisir un commentaire composé d'autant de lignes qu'il le souhaite, jusqu'à la limite de caractères par défaut.

Cependant, il peut être souhaitable de limiter l'utilisateur à la saisie d'une seule ligne de texte. Des exemples d'utilisation incluent les enchères en ligne ou le clavardage en direct, pour lesquels FastComments peut être utilisé.

Nous activons le drapeau **useSingleLineCommentInput** comme suit :

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, consultez la section « Enable Single-Line Comment Input ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque ordre de tri sont pré-calculés, donc tous les ordres de tri offrent les mêmes performances.

---