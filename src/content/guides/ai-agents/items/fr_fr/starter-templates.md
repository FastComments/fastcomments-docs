FastComments fournit cinq modèles de démarrage afin que vous n'ayez pas à écrire un agent fonctionnel à partir de zéro. Ils sont accessibles depuis la [page AI Agents](https://fastcomments.com/auth/my-account/ai-agents) en cliquant sur **Browse templates**.

Lorsque vous choisissez un modèle :

1. L'agent est créé avec le **Statut : Dry Run** et un nom interne basé sur le modèle (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Si ce nom est déjà pris sur votre tenant, un suffixe numérique est ajouté.
2. Vous arrivez directement sur le formulaire d'édition avec tout pré-rempli - prompt, déclencheurs, actions autorisées et éventuels seuils. Une bannière en haut de page indique « Créé à partir du modèle {templateName}. Vérifiez les paramètres ci-dessous, puis passez le statut à Enabled lorsque vous êtes prêt. »
3. Rien n'est encore activé. L'agent n'agira pas tant que vous n'aurez pas enregistré et conservé le mode dry-run (pour observer) ou basculé sur Enabled.

### Les cinq modèles

- **[Moderator](#template-moderator)** - examine les commentaires nouveaux et signalés, avertit les contrevenants de première infraction, et n'escalade vers un bannissement qu'après un avertissement. Se déclenche sur les nouveaux commentaires et au franchissement du seuil de signalement (seuil par défaut : 3). Outils autorisés : `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - répond chaleureusement aux nouveaux commentateurs avec un accueil court et personnalisé. Se déclenche sur le premier commentaire d'un nouvel utilisateur. Outil autorisé : `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - épingle les commentaires de premier niveau substantiels une fois qu'ils franchissent un seuil de votes (par défaut : 10), en désépinglant d'abord le commentaire précédemment épinglé. Se déclenche au franchissement du seuil de votes. Outils autorisés : `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publie un résumé neutre d'un seul paragraphe sur les longs fils de discussion après un délai, puis l'épingle. Se déclenche sur les nouveaux commentaires avec un report de 30 minutes afin que le fil se stabilise avant la synthèse. Outils autorisés : `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - surveille les modifications de commentaires pour détecter les réécritures en cours de fil qui dénaturent les réponses, restaure le texte original et envoie un message privé à l'auteur. Se déclenche sur les modifications de commentaires. Outils autorisés : `edit_comment`, `warn_user`, `send_dm`.

### Personnaliser un modèle

Les modèles sont des points de départ, pas des contrats. Vous êtes invité à :

- Ajuster le **prompt initial** pour qu'il corresponde au ton de votre communauté.
- Ajouter ou supprimer des **déclencheurs** selon la fréquence d'exécution souhaitée de l'agent.
- Ajouter des **approbations** pour toute action sensible - nous recommandons fortement de soumettre `ban_user` à approbation pour les modèles de type modérateur.
- Ajouter des **règles communautaires** afin que l'agent applique votre politique écrite de manière cohérente. Voir [Règles communautaires](#community-guidelines).
- Définir des **budgets** par agent adaptés au nombre de déclenchements attendus.

Le modèle n'est qu'un véhicule qui pré-remplit des valeurs par défaut pertinentes ; une fois enregistré, l'agent vous appartient.