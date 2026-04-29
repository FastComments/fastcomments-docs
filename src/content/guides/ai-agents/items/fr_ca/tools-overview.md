Les **outils** d'un agent sont les actions qu'il peut entreprendre. Le formulaire d'édition de l'agent comporte une section **Allowed tool calls** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approvals** où vous cochez les actions qui devraient nécessiter l'approbation d'un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Disallowed** - l'agent ne peut pas le voir ni l'utiliser.
- **Allowed, no approval** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Allowed, with approval** - l'appel de l'agent est mis en file d'attente pour révision humaine et n'est exécuté que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas les demander et la plateforme les refuse catégoriquement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit pour chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1-2 phrases expliquant pourquoi) et un score de confiance (0.0-1.0). Les deux apparaissent dans la [Vue des détails d'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et est toujours disponible indépendamment de l'allowlist.

### Référence des outils

#### Publication de commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents de bienvenue et de résumé. Réversible - tout modérateur peut supprimer un commentaire inapproprié. Généralement autorisé sans approbation ; mettez-le sous approbation si votre communauté a besoin que chaque message public soit relu par un humain.

#### Modification d'un commentaire

Permet à l'agent de réécrire le texte d'un commentaire inclus dans le périmètre. Le texte original est conservé dans le journal d'audit du commentaire. À réserver à des cas précis - expurger des informations personnelles qu'un utilisateur a divulguées, ou corriger la réponse antérieure de l'agent. Pas pour réécrire des opinions ou adoucir le ton. **Envisagez fortement de le placer derrière une approbation.** Voir [Modifier un commentaire](#tool-edit-comment) pour la page complète.

#### Vote sur les commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total des votes du commentaire comme tout autre vote. La plupart des communautés préfèrent que des bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en tête de page ou de désépingler un commentaire déjà épinglé. La plateforme n'impose pas de règle d'une épingle par fil, donc un agent chargé d'épingler devrait être instruit de désépingler d'abord le commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher d'autres réponses sous un commentaire, ou de rétablir les réponses. Le commentaire verrouillé reste visible. Utile pour calmer des fils houleux, associé à un déverrouillage différé. Réversible mais visible pour votre communauté ; envisagez de le soumettre à approbation dans les communautés à enjeux élevés.

#### Marquer / démarquer comme spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou de supprimer ce drapeau. L'outil de base pour tout agent de modération. Réversible. Envisagez fortement de le placer sous approbation durant les premières semaines pendant que vous établissez la confiance envers l'agent.

#### Approuver / retirer l'approbation d'un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les tenants qui retiennent les nouveaux commentaires pour révision par un modérateur. À enjeux élevés lorsqu'il s'agit de retirer l'approbation d'un commentaire visible - envisagez de le placer sous approbation.

#### Marquer un commentaire comme examiné

Outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) a examiné ceci ». Ne change pas la visibilité. Faible enjeu ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer à un utilisateur un badge provenant de la configuration des badges de votre tenant. Réversible par un modérateur. Rarement soumis à approbation. L'agent doit connaître l'ID du badge, donc incluez les ID pertinents dans vos [directives communautaires](#community-guidelines) ou dans l'[invite initiale](#personality-prompt).

#### Envoyer un courriel

Permet à l'agent d'envoyer un courriel en texte brut depuis `noreply@fastcomments.com` à une adresse qu'il choisit. À utiliser avec parcimonie - le courriel est l'outil le plus à friction élevée et les mauvais courriels sont difficiles à annuler. Envisagez fortement de le placer sous approbation, et orientez les courriels d'approbation vers la personne qui gère la boîte de réception que l'agent finira par utiliser.

#### Enregistrer / rechercher dans la mémoire de l'agent

Deux outils appariés qui lisent et écrivent un pool de notes partagé à propos de l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre tenant, donc les notes d'un agent de triage informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Système de mémoire des agents](#agent-memory-system) pour la conception complète.

#### Avertir un utilisateur

Envoie un message privé d'avertissement à un utilisateur au sujet d'un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil - avertir d'abord, bannir seulement si l'utilisateur récidive. Moins souvent soumis à approbation que `ban_user`, mais envisagez de le soumettre à approbation pendant les premières semaines de vie d'un agent. Voir [Avertir l'utilisateur](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus conséquent qu'un agent peut appeler. Banni un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant également l'IP, éventuellement en supprimant aussi tous les commentaires de l'utilisateur. Les deux options destructrices (IP, delete-all) sont soumises à des opt-ins supplémentaires sur le formulaire d'édition. Dans la région UE, tous les bannissements requièrent l'approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Envisagez fortement de le placer sous approbation partout. Voir [Bannir un utilisateur](#tool-ban-user) pour la page complète.

### Sous-options de l'outil Bannir

L'outil Bannir expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont cachées au modèle entièrement jusqu'à ce que vous les activiez via la section **Ban options** sur le formulaire d'édition. Même si le modèle hallucine le paramètre, la plateforme refuse les valeurs pour lesquelles vous n'avez pas donné votre accord. Voir [Bannir un utilisateur](#tool-ban-user).