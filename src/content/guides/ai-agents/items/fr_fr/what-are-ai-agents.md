Un **Agent IA** est un travailleur autonome, lié à votre tenant FastComments, qui surveille les événements dans votre communauté et agit en votre nom.

Chaque agent a trois éléments que vous contrôlez :

1. **Une personnalité.** Une invite initiale en texte libre qui définit le ton, le rôle et le style de prise de décision (« Vous êtes un accueillant chaleureux de la communauté », « Vous appliquez les règles communautaires mais privilégiez l'avertissement plutôt que le bannissement », etc.).
2. **Un ou plusieurs déclencheurs.** Une liste d'événements qui réveillent l'agent - un nouveau commentaire, un commentaire dépassant un seuil de votes ou de signalements, une action de modérateur, le premier commentaire d'un utilisateur sur le site, et d'autres. La liste complète est dans [Aperçu des événements déclencheurs](#triggers-overview).
3. **Une liste autorisée d'outils.** Ce que l'agent est autorisé à faire - publier un commentaire, voter, épingler, verrouiller, marquer comme spam, bannir un utilisateur, avertir via DM, attribuer un badge, envoyer un e-mail, sauvegarder et rechercher une mémoire partagée. La liste complète est dans [Aperçu des appels d'outils autorisés](#tools-overview).

Lorsqu'un déclencheur s'active, l'agent reçoit un message de contexte décrivant ce qui s'est passé (le commentaire, la page, le contexte facultatif du fil/utilisateur/page) et reçoit son invite initiale ainsi que vos règles communautaires. Il appelle ensuite des outils pour agir, en enregistrant une justification et un score de confiance à chaque appel.

### Les agents s'exécutent de façon asynchrone

Les agents **n'empêchent jamais l'action de l'utilisateur qui les a déclenchés**. Un lecteur publie un commentaire, le commentaire est enregistré et affiché dans le fil, la réponse est renvoyée, et ce n'est *qu'ensuite* que l'agent s'exécute sur celui-ci - soit immédiatement, soit après un délai configuré (voir [Déclencheurs différés](#trigger-deferred-delay)). Rien de ce que fait l'agent n'ajoute de latence à l'expérience utilisateur.

### Pourquoi les utiliser

- **Modérer à grande échelle.** Marquer le spam évident et bannir les récidivistes sans surveiller la file d'attente en permanence.
- **Accueillir les nouveaux commentateurs.** Répondre aux nouveaux commentateurs dans votre ton.
- **Mettre en avant le meilleur contenu.** Épingler les commentaires de premier niveau substantiels une fois qu'ils dépassent un seuil de votes.
- **Appliquer vos directives de manière cohérente.** Appliquer le même texte de politique à chaque commentaire limite.
- **Résumer les longs fils de discussion.** Publier des résumés neutres de débats s'étalant sur plusieurs pages.

### Ce qui vous permet de garder le contrôle

- **Mode Dry-Run.** Chaque nouvel agent est livré en **Mode Dry-Run** : il traite les déclencheurs, exécute le modèle, et enregistre ce qu'il *ferait*, mais n'entreprend aucune action réelle. Voir [Mode Dry-Run](#dry-run-mode).
- **Approbations.** N'importe quel sous-ensemble d'actions peut être soumis à une approbation humaine. Voir [Flux d'approbation](#approval-workflow).
- **Budgets par agent et par compte.** Plafonds stricts journaliers et mensuels. Voir [Aperçu des budgets](#budgets-overview).
- **Liste autorisée d'outils.** Les outils non autorisés sont retirés de la palette du modèle - l'agent ne peut littéralement pas les demander. Voir [Aperçu des appels d'outils autorisés](#tools-overview).
- **Champs d'audit sur chaque action.** Le modèle doit inclure une justification et un score de confiance. Les deux apparaissent dans la chronologie d'exécution et sur chaque approbation. Voir [Vue détaillée de l'exécution](#run-detail-view).
- **Article 17 du DSA de l'UE.** Dans la région UE, les bannissements entièrement automatisés sont bloqués. Voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance).
- **Pas d'entraînement sur vos données.** FastComments utilise des fournisseurs qui n'entraînent pas leurs modèles sur vos invites ou vos commentaires.

### Où ils s'intègrent à la modération humaine

Les agents et les modérateurs humains partagent la même plateforme de commentaires : les agents effectuent des actions via les mêmes canaux (approuver, marquer comme spam, bannir, attribuer un badge, épingler, verrouiller, écrire) et ces actions apparaissent dans les mêmes [Journaux des commentaires](/guide-moderation.html#comment-logs), la même [Page de modération](/guide-moderation.html#moderate-comments-page), et les mêmes flux de notification. Agents et humains voient le travail de l'autre et peuvent réagir l'un à l'autre - les actions des modérateurs sont elles-mêmes des déclencheurs valides pour les agents (voir [Déclencheur : commentaire examiné par un modérateur](#trigger-moderator-reviewed) et autres).