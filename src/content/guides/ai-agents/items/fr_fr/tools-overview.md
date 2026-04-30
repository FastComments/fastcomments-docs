Les **outils** d'un agent sont les actions qu'il peut entreprendre. Le formulaire d'édition de l'agent comporte une section **Appels d'outils autorisés** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approbations** où vous cochez les actions qui doivent être approuvées par un humain avant d'entrer en vigueur.

Il existe trois niveaux pour chaque outil :

- **Interdit** - l'agent ne peut pas le voir ni l'utiliser.
- **Autorisé, sans approbation** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Autorisé, avec approbation** - l'appel de l'agent est mis en file d'attente pour examen humain et ne s'exécute que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas les demander et la plateforme les refuse purement et simplement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit sur chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1–2 phrases expliquant pourquoi) et un score de confiance (0.0–1.0). Les deux apparaissent dans la [Vue détaillée d'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et reste toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Publier des commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible - tout modérateur peut supprimer un commentaire inapproprié. Placez-le derrière une approbation si votre communauté a besoin que chaque message public soit relu par un humain.

#### Modifier un commentaire

Permet à l'agent de réécrire le texte d'un commentaire dans le périmètre. Le texte original est conservé dans le journal d'audit du commentaire. À réserver aux cas limités - effacer des PII qu'un utilisateur a divulgué, ou modifier la propre réponse antérieure de l'agent. Pas pour réécrire des opinions ou atténuer le ton. Voir [Modifier un commentaire](#tool-edit-comment) pour la page complète.

#### Voter sur les commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total comme n'importe quel autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou de désépingler un commentaire déjà épinglé. La plateforme n'applique pas de règle d'une seule épingle par fil, donc un agent chargé d'épingler devrait être instruit de désépingler d'abord le commentaire précédemment épinglé. Pour découvrir ce qui est déjà épinglé sur la même page, l'agent peut appeler l'outil en lecture seule `get_pinned_comments` (voir ci-dessous). Utilisé par le modèle Top Comment Pinner.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher d'autres réponses sous un commentaire, ou de restaurer la possibilité de répondre. Le commentaire verrouillé reste visible. Utile pour calmer des fils houleux, associé à un déverrouillage différé. Pour découvrir ce qui est actuellement verrouillé sur la même page, l'agent peut appeler l'outil en lecture seule `get_locked_comments` (voir ci-dessous).

#### Marquer / retirer le marquage de spam

Permet à l'agent de marquer un commentaire comme spam (le rendant invisible aux lecteurs et alimentant le classificateur de spam) ou de lever ce signalement. L'outil de base pour tout agent de modération. Réversible.

#### Approuver / retirer l'approbation d'un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Particulièrement utile sur les tenants qui retiennent les nouveaux commentaires pour examen par un modérateur.

#### Marquer un commentaire comme examiné

Outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) a consulté ceci ». Ne change pas la visibilité. Peu risqué ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer à un utilisateur un badge que vous avez configuré pour votre tenant. Réversible par un modérateur. Lorsque cet outil est activé, l'agent peut voir les badges de votre tenant et choisir le bon de lui‑même, vous n'avez donc pas besoin d'insérer des identifiants de badge dans vos directives communautaires ou votre prompt initial. Pour orienter quel badge est attribué pour quel comportement, référez-vous aux badges par leur **Display Label** dans le prompt.

#### Envoyer un e-mail

Permet à l'agent d'envoyer un e-mail en texte brut à l'auteur d'un commentaire dans le périmètre du déclencheur. L'agent ne voit jamais l'adresse e-mail du destinataire : il choisit un commentaire et la plateforme livre à l'adresse que l'auteur a fournie lors de sa publication. L'adresse d'expédition est l'expéditeur marqué de votre tenant (avec DKIM) lorsque le domaine du commentaire correspond à un domaine configuré ; sinon, c'est l'expéditeur par défaut de la plateforme. À utiliser avec parcimonie : l'e-mail est l'outil le plus contraignant et les mauvais e-mails sont difficiles à annuler.

#### Enregistrer / rechercher dans la mémoire de l'agent

Deux outils appariés qui lisent et écrivent dans un pool de notes partagé concernant l'utilisateur pour lequel un déclencheur s'est exécuté. La mémoire est partagée entre tous les agents de votre tenant, ainsi les notes d'un agent de tri informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Système de mémoire des agents](#agent-memory-system) pour la conception complète.

#### Lister les commentaires épinglés / Lister les commentaires verrouillés

Deux outils de découverte en lecture seule qui listent les commentaires épinglés (ou verrouillés) sur la même page (`urlId`) où le déclencheur s'est produit. Ils ne prennent aucun argument : la page est lue à partir du contexte du déclencheur, l'agent ne peut donc pas pivoter vers d'autres pages. Utilisez-les quand un agent doit agir sur un commentaire déjà épinglé ou verrouillé - typiquement l'appel initial avant `unpin_comment` ou `unlock_comment`, ou avant d'épingler un nouveau commentaire afin de pouvoir désépingler d'abord l'existant.

Chaque outil est verrouillé séparément dans **Appels d'outils autorisés** (l'administrateur coche `List pinned comments on the current page` ou `List locked comments on the current page`). Ils ne peuvent pas être soumis à approbation - les outils en lecture seule n'ont pas d'effet de bord à approuver. Les appeler n'est pas enregistré comme une action dans l'historique d'exécution ; seule l'éventuelle action résultante `unpin_comment` / `unlock_comment` / `pin_comment` apparaît. La liste est limitée aux 20 correspondances les plus récentes par appel.

Important à comprendre : lorsqu'un de ces outils renvoie un commentId, ce commentId est ajouté au périmètre par exécution de l'agent, de sorte que l'appel de suivi `unpin_comment` / `unlock_comment` est validé par la vérification de sécurité des cibles d'outil de la plateforme. Sans avoir d'abord appelé l'outil de découverte, l'agent ne peut pas agir sur des commentaires qui ne sont pas déjà dans le périmètre immédiat du déclencheur. Ainsi, un agent de type désépinglage obtient typiquement les deux outils activés (par ex. `get_pinned_comments` plus `unpin_comment`).

#### Avertir un utilisateur

Envoie un message privé (DM) d'avertissement à un utilisateur au sujet d'un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil : avertir d'abord, bannir uniquement en cas de récidive. Voir [Avertir l'utilisateur](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus lourd de conséquences qu'un agent puisse appeler. Banni un utilisateur pour une durée fixe, éventuellement en shadow ban, optionnellement en bannissant aussi l'IP, optionnellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, suppression de tous les commentaires) sont protégées par des opt-ins supplémentaires sur le formulaire d'édition. Dans la région UE, tous les bannissements requièrent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Voir [Bannir un utilisateur](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil de bannissement expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont cachées au modèle tant que vous ne les avez pas activées via la section **Ban options** sur le formulaire d'édition. Même si le modèle invente le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Bannir un utilisateur](#tool-ban-user).