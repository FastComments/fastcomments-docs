**Template ID:** `welcome_greeter`

Le Welcome Greeter répond chaleureusement aux personnes qui commentent pour la première fois. C'est le modèle le moins risqué (aucun outil destructeur) et un bon premier agent à déployer en production.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Déclencheurs

- **Un nouvel utilisateur publie son premier commentaire sur ce site** (`NEW_USER_FIRST_COMMENT`).

Cet événement se déclenche exactement une fois par utilisateur, donc l'agent ne peut pas boucler. Voir [Déclencheur : Premier commentaire d'un nouvel utilisateur](#trigger-new-user-first-comment).

### Outils autorisés

- [`write_comment`](#tools-overview)

C'est le seul outil - l'agent ne peut littéralement pas modérer, voter, bannir ou envoyer des messages privés.

### Ajouts recommandés avant la mise en production

- **Définissez le nom d'affichage** sur quelque chose d'accueillant - "Community Bot", la mascotte de votre site, ou le nom de votre marque. Le nom d'affichage est ce que les lecteurs voient attaché à la réponse de bienvenue.
- **Cochez "Include page title, subtitle, description, and meta tags"** dans [Context Options](#context-options). Les réponses du greeter s'améliorent nettement lorsqu'il peut référencer le sujet réel de la page.
- **Envisagez des restrictions de langue** si vous opérez dans plusieurs langues. Une réponse de bienvenue dans la mauvaise langue est plus perturbante qu'une réponse manquée. Voir [Périmètre : filtres d'URL et de langue](#scope-url-locale).

### Pourquoi aucune approbation n'est nécessaire

L'agent écrit uniquement de nouveaux commentaires et uniquement suite à un déclencheur ponctuel. Dans le pire des cas : une salutation maladroite. Il n'y a aucune action destructive à contrôler. La plupart des exploitants exécutent celui-ci sans aucune approbation une fois que l'exécution à blanc est concluante.

---