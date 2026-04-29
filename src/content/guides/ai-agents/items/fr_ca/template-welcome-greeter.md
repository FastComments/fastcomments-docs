---
**ID du modèle:** `welcome_greeter`

Le Welcome Greeter répond chaleureusement aux personnes qui commentent pour la première fois. C'est le modèle le moins risqué (aucun outil destructeur) et un bon premier agent à déployer en production.

### Déclencheurs

- **Un nouvel utilisateur publie son premier commentaire sur ce site** (`NEW_USER_FIRST_COMMENT`).

Cet événement se déclenche exactement une fois par utilisateur, donc l'agent ne peut pas boucler. Voir [Déclencheur : Premier commentaire d'un nouvel utilisateur](#trigger-new-user-first-comment).

### Outils autorisés

- [`write_comment`](#tools-overview)

C'est le seul outil — l'agent ne peut littéralement pas modérer, voter, bannir ou envoyer un message direct (DM).

### Ajouts recommandés avant la mise en ligne

- **Définissez le nom affiché** sur quelque chose d'invitant — "Community Bot", la mascotte de votre site, ou le nom de votre marque. Le nom affiché est ce que les lecteurs voient attaché à la réponse de bienvenue.
- **Cochez "Inclure le titre de la page, le sous-titre, la description et les balises méta"** dans [Options de contexte](#context-options). Les réponses du greeter s'améliorent nettement lorsqu'il peut faire référence à ce dont la page parle réellement.
- **Envisagez des restrictions de paramètres régionaux** si vous opérez dans plusieurs langues. Une réponse de bienvenue dans la mauvaise langue est plus gênante qu'une réponse manquée. Voir [Portée : filtres d'URL et de paramètres régionaux](#scope-url-locale).

### Pourquoi aucune approbation n'est nécessaire

L'agent n'écrit que de nouveaux commentaires et seulement en réaction à un déclencheur ponctuel. Dans le pire des cas : un accueil maladroit. Il n'y a aucune action destructive à contrôler. La plupart des opérateurs exécutent celui-ci sans aucune approbation une fois que l'exécution en simulation semble satisfaisante.

---