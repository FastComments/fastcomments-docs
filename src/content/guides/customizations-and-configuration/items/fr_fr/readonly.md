[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Les commentaires peuvent être verrouillés afin qu'aucun nouveau commentaire ou vote ne puisse être laissé en définissant le drapeau readonly sur true.

Les commentaires ne pourront également pas être modifiés ou supprimés.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, pour un domaine entier ou une page :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Paramètre empêchant les nouvelles réponses sur la page de personnalisation du widget, qui verrouille un fil pour un domaine ou une page'; title='Rendre le fil de commentaires en lecture seule' app-screenshot-end]

## Mise à jour !

Depuis novembre 2022, les fils peuvent être verrouillés ou déverrouillés **en direct** par les administrateurs et les modérateurs via le menu à trois points au-dessus de la zone de réponse.

Cela empêchera les nouveaux commentaires, tout en permettant le vote et en autorisant les utilisateurs à supprimer leurs commentaires si désiré, alors que `readonly` ne permet pas ces actions. 

Cela correspond au champ `isClosed` de l'API `Page`.