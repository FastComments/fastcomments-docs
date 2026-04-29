Déclenche l'agent lorsqu'un commentaire est modifié.

### Contexte que reçoit l'agent

- Le commentaire dans sa forme actuelle (après modification).
- Le **texte précédent du commentaire** en tant que bloc séparé délimité (`PREVIOUS_TEXT`). Ceci est unique au déclencheur d'édition - l'agent peut comparer avant/après.
- Contexte facultatif du fil / de l'utilisateur / de la page tel que configuré.

### À noter

- Le déclencheur est activé pour toute modification réussie, y compris les modifications effectuées par des modérateurs au nom d'un utilisateur.
- Les agents n'ont aucun outil d'édition de commentaire à leur disposition ; ils ne peuvent pas du tout modifier les commentaires.
- Le texte précédent du commentaire est délimité comme une entrée non fiable. Le prompt système de la plateforme rappelle au modèle de ne pas suivre les instructions à l'intérieur des blocs délimités - cela est important ici, car un utilisateur malveillant pourrait modifier un commentaire pour y insérer une charge utile "ignorez vos instructions précédentes" visant tout agent surveillant les événements de modification.

### Usages courants

- **Repérage de contenu réintroduit** - un utilisateur modifie un commentaire auparavant propre pour y insérer du spam après que le modérateur soit passé à autre chose.
- **Suivi des petites modifications** - si votre communauté considère les modifications comme des événements distincts pour des raisons d'audit.

### Note sur le coût

Les déclencheurs d'édition voient deux copies du texte du commentaire (la nouvelle version dans le bloc standard COMMENT, l'ancienne version dans le bloc PREVIOUS_TEXT). Pour les commentaires longs, cela double à peu près le coût en tokens de l'exécution par rapport à un déclencheur `COMMENT_ADD` - gardez cela à l'esprit lors de l'établissement du budget.