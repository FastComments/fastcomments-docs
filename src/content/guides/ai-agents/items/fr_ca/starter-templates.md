FastComments fournit quatre modèles de démarrage afin que vous n'ayez pas à créer un agent fonctionnel à partir de zéro. Ils sont accessibles depuis la [page Agents IA](https://fastcomments.com/auth/my-account/ai-agents) en cliquant sur **Parcourir les modèles**.

Lorsque vous choisissez un modèle :

1. L'agent est créé avec **Statut : Mode simulation** et un nom interne basé sur le modèle (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Si ce nom est déjà utilisé sur votre locataire, un suffixe numérique est ajouté.
2. Vous arrivez directement sur le formulaire d'édition avec tout prérempli : prompt, triggers, actions autorisées et tous les seuils. Une bannière en haut indique "Créé à partir du modèle {templateName}. Vérifiez les paramètres ci-dessous, puis changez le statut en Activé lorsque vous êtes prêt."
3. Rien n'est encore activé. L'agent n'agira pas tant que vous n'aurez pas enregistré et soit conservé le mode simulation activé (pour observer), soit basculé sur Activé.

### Les quatre modèles

- **[Moderator](#template-moderator)** - examine les new comments et les commentaires signalés, avertit les contrevenants pour la première fois, n'escalade au bannissement qu'après un avertissement. Se déclenche sur new comments et sur flag-threshold crossings (par défaut : flag threshold = 3). Outils autorisés : `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - répond chaleureusement aux commentateurs pour la première fois avec un court message de bienvenue personnalisé. Se déclenche sur new-user-first-comment. Outil autorisé : `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - épingle les commentaires substantiels de niveau supérieur une fois qu'ils dépassent un seuil de votes (par défaut : 10), en désépinglant d'abord le commentaire précédemment épinglé. Se déclenche sur vote-threshold crossings. Outils autorisés : `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publie un résumé neutre en un seul paragraphe sur les fils longs après un délai, puis l'épingles. Se déclenche sur new comments avec un report de 30 minutes afin que le fil se stabilise avant la synthèse. Outils autorisés : `write_comment`, `pin_comment`, `unpin_comment`.

### Personnaliser un modèle

Les modèles sont des points de départ, pas des contrats. Il est attendu que vous :

- Ajustiez le **Initial prompt** pour qu'il corresponde à la voix de votre communauté.
- Ajoutiez ou supprimiez des **Triggers** pour adapter la fréquence d'exécution de l'agent.
- Ajoutiez des **Approvals** pour toute action sensible — nous recommandons fortement de placer `ban_user` derrière une approbation pour les modèles de type modérateur.
- Ajoutiez des **Community guidelines** afin que l'agent applique votre politique écrite de façon cohérente. Voir [Community Guidelines](#community-guidelines).
- Définissiez des **Budgets** par agent appropriés au nombre de triggers que vous prévoyez.

Le modèle n'est qu'un véhicule qui préremplit des valeurs par défaut sensées ; une fois enregistré, l'agent vous appartient.