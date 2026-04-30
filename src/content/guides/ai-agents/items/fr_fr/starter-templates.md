FastComments fournit cinq modèles de démarrage afin que vous n'ayez pas à écrire un agent fonctionnel depuis zéro. Ils sont accessibles depuis la [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) en cliquant sur **Browse templates**.

Lorsque vous choisissez un modèle :

1. L'agent est créé avec **Statut : Exécution à blanc** et un nom interne basé sur le modèle (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Si ce nom est déjà utilisé sur votre tenant, un suffixe numérique est ajouté.
2. Vous arrivez directement sur le formulaire d'édition avec tout pré-rempli - prompt, déclencheurs, actions autorisées et éventuels seuils. Une bannière en haut indique "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Rien n'est encore activé. L'agent n'agira pas tant que vous n'aurez pas enregistré et soit laissé l'exécution à blanc activée (pour observer), soit basculé sur Enabled.

### The five templates

- **[Modérateur](#template-moderator)** - examine les nouveaux commentaires et ceux signalés, avertit les contrevenants pour la première fois, n'escalade vers un bannissement qu'après un avertissement. Se déclenche sur les nouveaux commentaires et lors du dépassement du seuil de signalement (seuil de signalement par défaut : 3). Outils autorisés : `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Accueil des nouveaux](#template-welcome-greeter)** - répond chaleureusement aux commentateurs pour la première fois avec un court message de bienvenue personnalisé. Se déclenche sur new-user-first-comment. Outil autorisé : `write_comment`.

- **[Épingleur des meilleurs commentaires](#template-top-comment-pinner)** - épingle les commentaires substantiels de niveau supérieur une fois qu'ils dépassent un seuil de votes (par défaut : 10), en désépinglant d'abord le commentaire précédemment épinglé. Se déclenche lors du dépassement du seuil de votes. Outils autorisés : `pin_comment`, `unpin_comment`.

- **[Résumé de discussion](#template-thread-summarizer)** - publie un résumé neutre en un seul paragraphe sur les longues discussions après un délai, puis l'épinglé. Se déclenche sur les nouveaux commentaires avec un report de 30 minutes pour que la discussion se stabilise avant le résumé. Outils autorisés : `write_comment`, `pin_comment`, `unpin_comment`.

- **[Détecteur de gaslighting](#template-gaslight-detector)** - surveille les modifications de commentaires pour détecter les réécritures en cours de fil qui déforment les réponses, restaure le texte original et envoie un message direct à l'auteur. Se déclenche sur les modifications de commentaires. Outils autorisés : `edit_comment`, `warn_user`, `send_dm`.

### Personnaliser un modèle

Les modèles sont des points de départ, pas des contrats. Vous êtes amené à :

- Ajuster le **message initial** pour qu'il corresponde à la voix de votre communauté.
- Ajouter ou supprimer des **Déclencheurs** pour définir la fréquence d'exécution souhaitée de l'agent.
- Ajouter des **Approbations** pour toute action sensible - nous recommandons vivement de soumettre `ban_user` à une approbation pour les modèles de type modérateur.
- Ajouter des **Directives communautaires** afin que l'agent applique de façon cohérente votre politique écrite. Voir [Community Guidelines](#community-guidelines).
- Définir des **Budgets** par agent appropriés au nombre de déclencheurs que vous prévoyez.

Le modèle n'est qu'un véhicule qui pré-remplit des valeurs par défaut sensées ; une fois enregistré, l'agent vous appartient.