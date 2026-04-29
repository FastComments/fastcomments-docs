**ID du modèle :** `welcome_greeter`

La Welcome Greeter répond chaleureusement aux commentateurs qui commentent pour la première fois. C'est le modèle le moins risqué (aucun outil destructeur) et un bon premier agent à déployer en production.

### Déclencheurs

- **Un nouvel utilisateur publie son premier commentaire sur ce site** (`NEW_USER_FIRST_COMMENT`).

Cet événement se déclenche exactement une fois par utilisateur, donc l'agent ne peut pas boucler. Voir [Déclencheur : Premier commentaire d'un nouvel utilisateur](#trigger-new-user-first-comment).

### Outils autorisés

- [`write_comment`](#tools-overview)

C'est le seul outil — l'agent ne peut littéralement pas modérer, voter, bannir ou envoyer des messages privés.

### Ajouts recommandés avant la mise en ligne

- **Définissez le nom d'affichage** sur quelque chose d'accueillant — "Community Bot", la mascotte de votre site, ou le nom de votre marque. Le nom d'affichage est ce que les lecteurs voient attaché à la réponse de bienvenue.
- **Cochez «Inclure le titre de la page, le sous-titre, la description et les balises meta»** dans [Options de contexte](#context-options). Les réponses du greeter s'améliorent nettement lorsqu'il peut faire référence à ce dont la page traite réellement.
- **Envisagez des restrictions de locale** si vous opérez dans plusieurs langues. Une réponse de bienvenue dans la mauvaise langue est plus déconcertante qu'une réponse manquée. Voir [Scope: URL and Locale Filters](#scope-url-locale).

### Pourquoi aucune approbation n'est requise

L'agent n'écrit que de nouveaux commentaires et uniquement lors d'un déclencheur unique. Au pire : une salutation maladroite. Il n'y a pas d'action destructive à restreindre. La plupart des exploitants utilisent ce modèle sans aucune approbation une fois que l'exécution à blanc est concluante.