[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Le nombre de commentaires affiché en haut du widget de commentaires peut soit afficher tous les commentaires "de premier niveau", c'est-à-dire les réponses qui sont des réponses directement à la page ou à l'article lui-même, soit il peut s'agir d'un décompte de **tous** les commentaires imbriqués.

Par défaut, c'est `true` — il s'agit du décompte de ce dernier cas — tous les commentaires. Dans les anciennes versions du widget de commentaires la valeur par défaut est `false`.

Nous pouvons modifier le comportement, de sorte qu'il s'agisse d'un décompte de **tous** les commentaires imbriqués en définissant le drapeau **countAll** sur true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Si nous voulions que le décompte reflète uniquement les commentaires de premier niveau, nous réglons le drapeau sur false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Cela ne peut actuellement pas être personnalisé sans modifications du code.

---