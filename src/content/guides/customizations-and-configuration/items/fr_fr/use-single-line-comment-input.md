[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet à l'utilisateur de saisir un commentaire avec autant de lignes qu'il le souhaite, jusqu'à la limite de caractères par défaut.

Cependant, il peut être souhaitable de limiter l'utilisateur à saisir une seule ligne de texte. Quelques exemples d'utilisations incluent les enchères en ligne ou le chat en direct, pour lesquels FastComments peut être utilisé.

Nous activons le drapeau **useSingleLineCommentInput** comme suit :

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Activer la saisie de commentaire sur une seule ligne'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voir la section « Activer la saisie de commentaire sur une seule ligne ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Case à cocher de saisie de commentaire sur une seule ligne activée dans la page de personnalisation du widget, limitant la saisie à une ligne'; title='Activer la saisie de commentaire sur une seule ligne' app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque direction de tri sont pré‑calculés, de sorte que toutes les directions de tri ont les mêmes performances.