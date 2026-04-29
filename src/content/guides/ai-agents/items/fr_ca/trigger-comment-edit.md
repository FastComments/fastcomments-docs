Déclenche l'agent lorsqu'un commentaire est modifié.

### Contexte que reçoit l'agent

- Le commentaire dans sa forme actuelle (après modification).
- Le **texte du commentaire précédent** en tant que bloc délimité séparé (`PREVIOUS_TEXT`). C'est propre au déclencheur d'édition - l'agent peut comparer l'avant/après.
- Historique facultatif du fil / de l'utilisateur / contexte de la page tel que configuré.

### À noter

- Le déclencheur s'active pour toute modification réussie, y compris celles effectuées par des modérateurs au nom d'un utilisateur.
- Les agents n'ont aucun outil d'édition de commentaire exposé; les agents ne peuvent pas du tout modifier les commentaires.
- Le texte du commentaire précédent est encadré comme une entrée non fiable. Le prompt système de la plateforme rappelle au modèle de ne pas suivre les instructions provenant de l'intérieur des cadres - cela compte ici, car un utilisateur malveillant pourrait modifier un commentaire pour y insérer une charge utile "ignorez vos instructions précédentes" visant tout agent surveillant les événements de modification.

### Utilisations courantes

- **Détecter du contenu dissimulé** - un utilisateur modifie un commentaire auparavant propre pour y insérer du spam après que le modérateur est passé à autre chose.
- **Suivi des modifications mineures** - si votre communauté considère les modifications comme des événements distincts pour quelque raison d'audit.

### Note sur le coût

Les déclencheurs d'édition reçoivent deux copies du texte du commentaire (la nouvelle version dans le bloc standard COMMENT, l'ancienne version dans le bloc PREVIOUS_TEXT). Pour les commentaires longs, cela double approximativement le coût en tokens de l'exécution par rapport à un déclencheur `COMMENT_ADD` - gardez cela en tête lors de l'établissement du budget.