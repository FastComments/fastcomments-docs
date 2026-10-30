Un **AI Agent** est un travailleur autonome, limité à votre locataire FastComments, qui surveille les événements de votre communauté et agit en votre nom.

Chaque agent possède trois éléments que vous contrôlez :

1. **Une personnalité.** Un texte libre initial qui définit le ton, le rôle et le style de prise de décision (« Vous êtes un accueillant chaleureux de la communauté », « Vous appliquez les règles de la communauté mais privilégiez l’avertissement à l’interdiction », etc.).
2. **Un ou plusieurs déclencheurs.** Une liste d’événements qui réveillent l’agent — un nouveau commentaire, un commentaire franchissant un seuil de votes ou de signalements, une action de modérateur, le premier commentaire d’un utilisateur sur le site, et d’autres. La liste complète se trouve dans **[Vue d’ensemble des événements déclencheurs](#triggers-overview)**.
3. **Une liste blanche d’outils.** Ce que l’agent est autorisé à faire — publier un commentaire, voter, épingler, verrouiller, marquer comme spam, bannir un utilisateur, avertir via DM, attribuer un badge, envoyer un e‑mail, sauvegarder et rechercher une mémoire partagée. La liste complète se trouve dans **[Vue d’ensemble des appels d’outils autorisés](#tools-overview)**.

Lorsqu’un déclencheur se produit, l’agent reçoit un message de contexte décrivant ce qui s’est passé (le commentaire, la page, le contexte optionnel du fil/utilisateur/page) et est invité avec son texte initial ainsi que vos directives communautaires. Il appelle ensuite des outils pour agir, en enregistrant une justification et un score de confiance à chaque appel.

### Les agents s’exécutent de façon asynchrone

Les agents **ne bloquent jamais l’action utilisateur qui les a déclenchés**. Un lecteur publie un commentaire, le commentaire est enregistré et affiché dans le fil, la réponse est renvoyée, et *ensuite* l’agent le traite — immédiatement ou après un délai configuré (voir **[Déclencheurs différés](#trigger-deferred-delay)**). Rien de ce que fait l’agent n’ajoute de latence à l’expérience utilisateur.

### Pourquoi les utiliser

- **Modérer à grande échelle.** Marquer le spam évident et bannir les récidivistes sans surveiller la file d’attente en permanence.
- **Accueillir les nouveaux commentateurs.** Répondre aux premiers commentaires dans votre ton.
- **Mettre en avant le meilleur contenu.** Épingler les commentaires de niveau supérieur substantiels dès qu’ils franchissent un seuil de votes.
- **Appliquer vos directives de façon cohérente.** Appliquer le même texte de politique à chaque commentaire borderline.
- **Résumer les longs fils.** Publier des résumés neutres de débats s’étalant sur plusieurs pages.

### Ce qui vous garde le contrôle

- **Mode simulation.** Chaque nouvel agent est livré en **Dry Run** : il traite les déclencheurs, exécute le modèle et enregistre ce qu’il *aurait* fait, mais n’effectue aucune action réelle. Voir **[Mode simulation](#dry-run-mode)**.
- **Approbations.** Un sous‑ensemble d’actions peut être soumis à une approbation humaine. Voir **[Flux de travail d’approbation](#approval-workflow)**.
- **Budgets par agent et par compte.** Plafonds quotidiens et mensuels stricts. Voir **[Vue d’ensemble des budgets](#budgets-overview)**.
- **Liste blanche d’outils.** Les outils non autorisés sont retirés de la palette du modèle — l’agent ne peut littéralement pas les demander. Voir **[Vue d’ensemble des appels d’outils autorisés](#tools-overview)**.
- **Champs d’audit sur chaque action.** Le modèle doit inclure une justification et un score de confiance. Les deux apparaissent dans la chronologie d’exécution et sur chaque approbation. Voir **[Vue détaillée de l’exécution](#run-detail-view)**.
- **Article 17 du DSA de l’UE.** Dans la région UE, les bannissements entièrement automatisés sont bloqués. Voir **[Conformité à l’article 17 du DSA de l’UE](#eu-dsa-compliance)**.
- **Aucune formation sur vos données.** FastComments utilise des fournisseurs qui ne s’entraînent pas sur vos invites ou commentaires.

### Où ils s’insèrent aux côtés de la modération humaine

Les agents et les modérateurs humains partagent la même plateforme de commentaires : les agents effectuent des actions via les mêmes canaux (approuver, spam, bannir, badge, épingler, verrouiller, écrire) et ces actions apparaissent dans les mêmes **[Journaux de commentaires](/guide-moderation.html#comment-logs)**, la même **[Page de modération](/guide-moderation.html#moderate-comments-page)** et les mêmes flux de notifications. Les agents et les humains voient le travail de l’autre et peuvent chacun réagir à l’autre — les actions de modérateur sont elles‑mêmes des déclencheurs valides pour les agents (voir **[Déclencheur : commentaire revu par un modérateur](#trigger-moderator-reviewed)** et similaires).

Si vous avez également besoin de personnes sur la file d’attente, FastComments propose **[Services de modération humaine](/guide-moderation.html#human-moderation-services)** : nos modérateurs travaillent aux côtés de vos agents, dans votre tableau de bord, sous vos directives.