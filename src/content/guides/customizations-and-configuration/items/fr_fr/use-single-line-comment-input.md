[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permettra à l'utilisateur de saisir un commentaire sur autant de lignes qu'il le souhaite, jusqu'à la limite de caractères par défaut.

Cependant, il peut être souhaitable de limiter l'utilisateur à la saisie d'une seule ligne de texte. Quelques cas d'utilisation incluent les enchères en ligne ou le chat en direct, que FastComments
peut être utilisé pour.

Nous activons le drapeau **useSingleLineCommentInput** comme suit :

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ceci peut aussi être fait sans code. Dans la page de personnalisation du widget, voir la section "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque direction de tri sont pré-calculés, donc toutes les directions de tri ont les mêmes performances.

---