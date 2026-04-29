Un **Agent IA** est un travailleur autonome, associé à votre instance FastComments, qui surveille les événements dans votre communauté et agit en votre nom.

Chaque agent dispose de trois éléments que vous contrôlez :

1. **Une personnalité.** Une invite initiale en texte libre qui définit le ton, le rôle et le style de prise de décision (« Vous êtes un accueillant chaleureux de la communauté », « Vous faites respecter les règles de la communauté mais privilégiez l'avertissement plutôt que l'interdiction », etc.).
2. **Un ou plusieurs déclencheurs.** Une liste d'événements qui réveillent l'agent - un nouveau commentaire, un commentaire franchissant un seuil de votes ou de signalements, une action d'un modérateur, le premier commentaire d'un utilisateur sur le site, et d'autres. La liste complète se trouve dans [Aperçu des événements déclencheurs](#triggers-overview).
3. **Une liste d'outils autorisés.** Ce que l'agent est autorisé à faire - poster un commentaire, voter, épingler, verrouiller, marquer comme spam, bannir un utilisateur, avertir via DM, attribuer un badge, envoyer un courriel, enregistrer et rechercher une mémoire partagée. La liste complète se trouve dans [Aperçu des appels d'outils autorisés](#tools-overview).

Lorsqu'un déclencheur se produit, l'agent reçoit un message de contexte décrivant ce qui s'est passé (le commentaire, la page, le contexte optionnel du fil/utilisateur/page) et il est sollicité avec son invite initiale et vos directives communautaires. Il appelle ensuite des outils pour agir, enregistrant une justification et un score de confiance à chaque appel.

### Les agents s'exécutent de façon asynchrone

Les agents **ne bloquent jamais l'action utilisateur qui les a déclenchés**. Un lecteur publie un commentaire, le commentaire est enregistré et affiché dans le fil, la réponse est renvoyée, et ce n'est qu'ensuite que l'agent l'analyse - soit immédiatement, soit après un délai configuré (voir [Déclencheurs différés](#trigger-deferred-delay)). Rien de ce que l'agent fait n'ajoute de latence à l'expérience visible par l'utilisateur.

### Pourquoi les utiliser

- **Modérer à grande échelle.** Marquez comme spam les contenus évidents et bannissez les récidivistes sans surveiller la file en continu.
- **Accueillir les nouveaux commentateurs.** Répondez aux commentateurs qui publient pour la première fois avec votre voix.
- **Faire ressortir le meilleur contenu.** Épinglez les commentaires substantiels de niveau supérieur une fois qu'ils franchissent un seuil de votes.
- **Appliquer vos directives de façon cohérente.** Appliquez le même texte de politique à chaque commentaire limite.
- **Résumer de longs fils de discussion.** Publiez des résumés neutres de débats sur plusieurs pages.

### Ce qui vous permet de garder le contrôle

- **Mode Dry Run.** Chaque nouvel agent est livré en **Mode Dry Run** : il traite les déclencheurs, exécute le modèle et enregistre ce qu'il *aurait* fait, mais n'effectue aucune action réelle. Voir [Mode Dry Run](#dry-run-mode).
- **Approbations.** N'importe quel sous-ensemble d'actions peut être soumis à une approbation humaine. Voir [Flux d'approbation](#approval-workflow).
- **Budgets par agent et par compte.** Plafonds stricts quotidiens et mensuels. Voir [Aperçu des budgets](#budgets-overview).
- **Liste d'outils autorisés.** Les outils non autorisés sont retirés de la palette du modèle - l'agent ne peut littéralement pas les demander. Voir [Aperçu des appels d'outils autorisés](#tools-overview).
- **Champs d'audit sur chaque action.** Le modèle doit inclure une justification et un score de confiance. Les deux apparaissent dans la chronologie d'exécution et sur chaque approbation. Voir [Vue détaillée de l'exécution](#run-detail-view).
- **Article 17 du DSA de l'UE.** Dans la région UE, les bannissements entièrement automatisés sont bloqués. Voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance).
- **Aucune utilisation de vos données pour l'entraînement.** FastComments utilise des fournisseurs qui n'entraînent pas leurs modèles sur vos invites ou vos commentaires.

### Comment ils s'intègrent à la modération humaine

Les agents et les modérateurs humains partagent la même plateforme de commentaires : les agents effectuent des actions par les mêmes canaux (approuver, marquer comme spam, bannir, attribuer un badge, épingler, verrouiller, rédiger) et ces actions apparaissent dans les mêmes [journaux de commentaires](/guide-moderation.html#comment-logs), la même [page de modération](/guide-moderation.html#moderate-comments-page) et les mêmes flux de notifications. Agents et humains voient le travail des uns et des autres et peuvent réagir l'un à l'autre - les actions des modérateurs constituent elles-mêmes des déclencheurs valides pour les agents (voir [Déclencheur : Commentaire examiné par un modérateur](#trigger-moderator-reviewed) et consorts).