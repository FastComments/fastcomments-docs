FastComments propose cinq modèles de démarrage afin que vous n'ayez pas à écrire un agent fonctionnel depuis zéro. Ils sont accessibles depuis la [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) en cliquant sur **Browse templates**.

Lorsque vous choisissez un modèle :

1. L'agent est créé avec **Statut : Exécution simulée** et un nom interne basé sur le modèle (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Si ce nom est déjà utilisé sur votre locataire, un suffixe numérique est ajouté.
2. Vous arrivez directement sur le formulaire d'édition avec tout prérempli - prompt, déclencheurs, actions autorisées et tout seuil. Une bannière en haut indique "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Rien n'est encore activé. L'agent n'agira pas tant que vous n'aurez pas enregistré et soit conservé le mode Exécution simulée activé (pour observer), soit basculé sur Activé.

### Les cinq modèles

- **[Modérateur](#template-moderator)** - examine les nouveaux commentaires et ceux signalés, avertit les contrevenants pour la première fois, n'escalade vers un bannissement qu'après un avertissement. Se déclenche sur les nouveaux commentaires et lorsque le seuil de signalement est dépassé (seuil de signalement par défaut : 3). Outils autorisés : `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Agent de bienvenue](#template-welcome-greeter)** - répond chaleureusement aux nouveaux commentateurs par un court message personnel de bienvenue. Se déclenche sur new-user-first-comment. Outil autorisé : `write_comment`.

- **[Épingleur du meilleur commentaire](#template-top-comment-pinner)** - épingle les commentaires substantiels de premier niveau une fois qu'ils dépassent un seuil de votes (par défaut : 10), en désépinglant d'abord le commentaire précédemment épinglé. Se déclenche sur vote-threshold crossings. Outils autorisés : `pin_comment`, `unpin_comment`.

- **[Résumé de fil](#template-thread-summarizer)** - publie un résumé neutre d'un seul paragraphe sur les fils longs après un délai, puis l'épingle. Se déclenche sur les nouveaux commentaires avec un report de 30 minutes afin que le fil se stabilise avant le résumé. Outils autorisés : `write_comment`, `pin_comment`, `unpin_comment`.

- **[Détecteur de gaslighting](#template-gaslight-detector)** - surveille les modifications de commentaires pour détecter les réécritures en cours de fil qui déforment les réponses, restaure le texte original et envoie un message direct à l'auteur. Se déclenche sur les modifications de commentaire. Outils autorisés : `edit_comment`, `warn_user`, `send_dm`.

### Personnaliser un modèle

Les modèles sont des points de départ, pas des contrats. Il est attendu que vous :

- Ajustiez l'**Invite initiale** pour correspondre à la voix de votre communauté.
- Ajoutiez ou supprimiez des **Déclencheurs** pour adapter la fréquence d'exécution de l'agent.
- Ajoutiez des **Approbations** pour toute action sensible - nous recommandons fortement de soumettre `ban_user` à approbation pour les modèles de type modérateur.
- Ajoutiez des **Directives communautaires** afin que l'agent applique de façon cohérente votre politique écrite. Voir [Directives communautaires](#community-guidelines).
- Définissiez des **Budgets** par agent appropriés au nombre de déclenchements que vous prévoyez.

Le modèle n'est qu'un véhicule qui préremplit des valeurs par défaut sensées ; une fois enregistré, l'agent vous appartient.