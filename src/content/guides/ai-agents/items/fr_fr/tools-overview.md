Les **outils** d'un agent sont les actions qu'il peut effectuer. Le formulaire d'édition de l'agent dispose d'une section **Allowed tool calls** où vous cochez les outils que cet agent est autorisé à utiliser, et d'une section **Approvals** où vous cochez les actions qui doivent être approuvées par un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Disallowed** - l'agent ne peut pas le voir ni l'utiliser.
- **Allowed, no approval** - l'agent l'utilise directement. Enregistré dans l'historique des exécutions.
- **Allowed, with approval** - l'appel de l'agent est mis en file d'attente pour révision humaine et ne s'exécute que lorsqu'un humain l'approuve.

Les outils non autorisés sont silencieux : l'agent ne peut pas les solliciter et la plateforme les refuse catégoriquement. Les outils soumis à approbation passent toujours par la [approvals inbox](#approval-workflow).

### Piste d'audit pour chaque action

Chaque action effectuée par l'agent est enregistrée avec une brève justification (1-2 phrases expliquant pourquoi) et un score de confiance (0.0-1.0). Les deux apparaissent dans la [Run Detail View](#run-detail-view) et sur chaque [approval](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et est toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Poster des commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible - tout modérateur peut supprimer un commentaire inapproprié. Généralement autorisé sans approbation ; restreignez-le si votre communauté exige que chaque message public soit relu par un humain.

#### Vote sur les commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total du commentaire comme n'importe quel autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans les modèles de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou de désépingler un commentaire déjà épinglé. La plateforme n'applique pas de règle d'une seule épingle par fil, donc un agent chargé d'épingler devrait être instruit de désépingler d'abord le commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher d'autres réponses sous un commentaire, ou de rétablir les réponses. Le commentaire verrouillé reste visible. Utile pour calmer des fils enflammés, associé à un déverrouillage différé. Réversible mais visible pour votre communauté ; envisagez de le soumettre à approbation dans les communautés à enjeux élevés.

#### Marquer / dé-marquer comme spam

Permet à l'agent de marquer un commentaire comme spam (le masquant aux lecteurs et alimentant le classificateur de spam) ou de supprimer ce marquage. L'outil fondamental pour tout agent de modération. Réversible. Envisagez fortement de le soumettre à approbation pendant les premières semaines pendant que vous construisez la confiance envers l'agent.

#### Approuver / retirer l'approbation d'un commentaire

Permet à l'agent de montrer un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les tenants qui retiennent les nouveaux commentaires pour révision par un modérateur. Risque élevé lorsqu'on retire l'approbation d'un commentaire visible - envisagez de le soumettre à approbation.

#### Marquer un commentaire comme examiné

Outil d'état de file d'attente : marque un commentaire comme "un modérateur (ou un agent) a regardé ceci". Ne change pas la visibilité. Faible risque ; rarement soumis à approbation.

#### Décerner un badge

Permet à l'agent d'attribuer à un utilisateur un badge provenant de la configuration des badges de votre tenant. Réversible par un modérateur. Rarement soumis à approbation. L'agent doit connaître l'ID du badge, donc incluez les IDs pertinents dans vos [community guidelines](#community-guidelines) ou dans votre [initial prompt](#personality-prompt).

#### Envoyer un e-mail

Permet à l'agent d'envoyer un e-mail en texte brut depuis `noreply@fastcomments.com` à une adresse qu'il choisit. Utilisez avec parcimonie : l'e-mail est l'outil le plus contraignant et les mauvais e-mails sont difficiles à annuler. Envisagez fortement de le soumettre à approbation, et orientez les e-mails d'approbation vers la personne qui possède la boîte de réception que l'agent va finir par contacter.

#### Enregistrer / rechercher la mémoire de l'agent

Deux outils complémentaires qui lisent et écrivent un pool de notes partagé concernant l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre tenant, ainsi les notes d'un agent de triage informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Agent Memory System](#agent-memory-system) pour la conception complète.

#### Avertir un utilisateur

Envoie un avertissement en DM privé à un utilisateur concernant un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil : avertir d'abord, bannir seulement si l'utilisateur récidive. Moins souvent soumis à approbation que `ban_user`, mais envisagez de le restreindre durant les premières semaines de vie d'un agent. Voir [Warn user](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus lourd de conséquences qu'un agent puisse appeler. Bannie un utilisateur pour une durée déterminée, éventuellement en shadow ban, éventuellement en bannissant aussi l'IP, éventuellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, delete-all) sont soumises à des opt-ins supplémentaires sur le formulaire d'édition. Même si le modèle invente le paramètre, la plateforme refuse les valeurs pour lesquelles vous n'avez pas opté. Voir [Ban user](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil Ban expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont complètement cachées au modèle tant que vous ne les avez pas activées via la section **Ban options** sur le formulaire d'édition. Même si le modèle hallucine le paramètre, la plateforme refuse les valeurs pour lesquelles vous n'avez pas opté. Voir [Ban user](#tool-ban-user).