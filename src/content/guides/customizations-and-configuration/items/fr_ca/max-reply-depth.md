[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Par défaut, FastComments permet un emboîtement illimité des réponses, créant une structure de fil où les utilisateurs peuvent répondre indéfiniment aux réponses.

L'option maxReplyDepth vous permet de limiter la profondeur maximale des fils de réponses. Lorsque la profondeur maximale est atteinte, les utilisateurs ne verront plus de bouton de réponse sur les commentaires à ce niveau.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Avec maxReplyDepth réglé à 2 :
- Les utilisateurs peuvent commenter au niveau supérieur (profondeur 0)
- Les utilisateurs peuvent répondre aux commentaires de niveau supérieur (profondeur 1)
- Les utilisateurs peuvent répondre à ces réponses (profondeur 2)
- Aucune réponse supplémentaire n'est autorisée au-delà de la profondeur 2

Le réglage à 1 ne permettrait que des réponses aux commentaires de niveau supérieur, créant une structure de discussion plus plate.

Définir maxReplyDepth à 0 désactiverait toutes les réponses, n'autorisant que les commentaires de niveau supérieur. Si non spécifié, les réponses peuvent être imbriquées sans limite.