FastComments propose quatre modèles de démarrage afin que vous n'ayez pas à écrire un agent fonctionnel à partir de zéro. Ils sont accessibles depuis la [page Agents AI](https://fastcomments.com/auth/my-account/ai-agents) en cliquant sur **Browse templates**.

When you pick a template:

1. L'agent est créé avec **Status: Dry Run** et un nom interne basé sur le modèle (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Si ce nom est déjà utilisé sur votre tenant, un suffixe numérique est ajouté.
2. Vous arrivez directement sur le formulaire d'édition avec tout prérempli — prompt, triggers, actions autorisées et éventuels seuils. Une bannière en haut indique "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Rien n'est encore activé. L'agent n'agira pas tant que vous n'aurez pas enregistré et soit gardé dry-run activé (pour observer), soit basculé sur Enabled.

### Les quatre modèles

- **[Moderator](#template-moderator)** - examine les nouveaux commentaires et les commentaires signalés, avertit les contrevenants pour la première fois, et n'escalade vers un bannissement qu'après un avertissement. Se déclenche sur les nouveaux commentaires et lors du franchissement d'un seuil de signalement (seuil de signalement par défaut : 3). Outils autorisés : `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - répond chaleureusement aux nouveaux commentateurs avec un court message personnel de bienvenue. Se déclenche sur new-user-first-comment. Outil autorisé : `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - épingle les commentaires de premier niveau substantiels une fois qu'ils ont dépassé un seuil de votes (par défaut : 10), en désépinglant d'abord le commentaire précédemment épinglé. Se déclenche lors du franchissement du seuil de votes. Outils autorisés : `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publie un résumé neutre en un seul paragraphe pour les fils longs après un délai, puis l'épingle. Se déclenche sur les nouveaux commentaires avec un report de 30 minutes afin que le fil se stabilise avant la synthèse. Outils autorisés : `write_comment`, `pin_comment`, `unpin_comment`.

### Personnaliser un modèle

Les modèles sont des points de départ, pas des contrats. Il est attendu que vous :

- Ajustiez le **Initial prompt** pour correspondre à la voix de votre communauté.
- Ajoutiez ou supprimiez des **Triggers** afin d'adapter la fréquence d'exécution de l'agent.
- Ajoutiez des **Approvals** pour toute action sensible — nous recommandons fortement de soumettre `ban_user` à approbation pour les modèles de type modération.
- Ajoutiez des **Community guidelines** afin que l'agent applique votre politique écrite de manière cohérente. Voir [Community Guidelines](#community-guidelines).
- Définissiez des **Budgets** par agent appropriés au nombre de triggers que vous prévoyez.

Le modèle n'est qu'un véhicule qui préremplit des valeurs par défaut sensées ; une fois enregistré, l'agent vous appartient.