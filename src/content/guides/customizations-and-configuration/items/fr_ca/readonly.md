[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

La possibilité de commenter peut être verrouillée de sorte qu'aucun nouveau commentaire ni vote ne puisse être laissé en définissant le drapeau readonly à true.

Les commentaires ne pourront également pas être modifiés ou supprimés.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, pour un domaine entier ou une page :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Mise à jour !

Depuis novembre 2022, les fils peuvent être verrouillés ou déverrouillés **en direct** par les administrateurs et les modérateurs via le menu à trois points au‑dessus de la zone de réponse.

Cela empêchera les nouveaux commentaires, tout en permettant toujours le vote et la suppression de leurs propres commentaires par les utilisateurs si désiré, alors que `readonly` n'autorise pas ces comportements. 

Cela correspond au champ `isClosed` dans l'API `Page`.

---