Les **outils** d'un agent sont les actions qu'il peut entreprendre. Le formulaire d'édition de l'agent comporte une section **Allowed tool calls** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approvals** où vous cochez les actions qui doivent être approuvées par un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Disallowed** - l'agent ne peut pas le voir ni l'utiliser.
- **Allowed, no approval** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Allowed, with approval** - l'appel de l'agent est mis en file d'attente pour révision humaine et n'est exécuté que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas en demander l'utilisation et la plateforme les refuse catégoriquement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit sur chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1–2 phrases expliquant pourquoi) et un score de confiance (0.0–1.0). Les deux apparaissent dans la [Vue détaillée de l'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme action et est toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Posting comments

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible : tout modérateur peut retirer un commentaire inapproprié. Généralement autorisé sans approbation ; soumettez-le à approbation si votre communauté exige qu’un humain révise chaque message public.

#### Editing a comment

Permet à l'agent de réécrire le texte d'un commentaire dans le périmètre autorisé. Le texte original est conservé dans le journal d'audit du commentaire. À réserver aux cas restreints : retirer des PII qu'un utilisateur a divulgué, ou modifier la réponse précédente de l'agent. Ne doit pas servir à réécrire des opinions ou à adoucir le ton. **Envisagez fortement de le soumettre à approbation.** Voir [Edit comment](#tool-edit-comment) pour la page complète.

#### Voting on comments

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total des votes du commentaire comme n’importe quel autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Pin / unpin a comment

Permet à l'agent d’épingler un commentaire en haut de la page ou de désépingler un commentaire déjà épinglé. La plateforme n’impose pas de règle d’un seul épinglé par fil, donc un agent qui épingle devrait être instruit de désépingler d’abord le commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Lock / unlock a comment

Permet à l'agent d'empêcher de nouvelles réponses sous un commentaire, ou de rétablir les réponses. Le commentaire verrouillé reste visible. Utile pour calmer des fils de discussion enflammés, souvent associé à un déverrouillage différé. Réversible mais visible par votre communauté ; envisagez une approbation sur les communautés à enjeux élevés.

#### Mark / unmark spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou de lever ce drapeau. L'outil de base pour tout agent de modération. Réversible. Envisagez fortement de le soumettre à approbation pendant les premières semaines le temps d'établir la confiance envers l'agent.

#### Approve / un-approve a comment

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les locataires qui retiennent les nouveaux commentaires pour révision par des modérateurs. Risqué lorsqu'on retire l'approbation d'un commentaire visible — envisagez une approbation humaine.

#### Mark a comment reviewed

Outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) a examiné ceci ». Ne change pas la visibilité. Faible enjeu ; rarement soumis à approbation.

#### Award a badge

Permet à l'agent d’attribuer à un utilisateur un badge que vous avez configuré pour votre tenant. Réversible par un modérateur. Rarément soumis à approbation. Lorsque cet outil est activé, l'agent peut voir les badges de votre tenant et choisir celui qui convient, vous n'avez donc pas besoin d'insérer des identifiants de badge dans vos directives communautaires ou votre prompt initial. Si vous souhaitez orienter quel badge est attribué pour quel comportement, faites référence aux badges par leur **Display Label** dans le prompt.

#### Send email

Permet à l'agent d'envoyer un courriel en texte brut à l'auteur d'un commentaire dans le périmètre du déclencheur. L'agent ne voit jamais l'adresse courriel du destinataire : il choisit un commentaire et la plateforme livre au courriel que l'auteur a fourni lors de sa publication. L'adresse d’expédition est l’expéditeur de marque de votre tenant (avec DKIM) lorsque le domaine du commentaire correspond à un domaine configuré, sinon c’est la valeur par défaut de la plateforme. Utilisez-le avec parcimonie : le courriel est l’outil le plus coûteux en termes de friction et les mauvais courriels sont difficiles à annuler. Envisagez fortement de le soumettre à approbation, et dirigez les courriels d’approbation vers la personne qui gère la boîte de réception que l’agent finira par contacter.

#### Save / search agent memory

Deux outils appariés qui lisent et écrivent dans un pool de notes partagé concernant l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre tenant, ainsi les notes d’un agent de triage informent les décisions d’un agent modérateur. La recherche est en lecture seule et toujours disponible ; l’enregistrement est rarement soumis à approbation. Voir [Agent Memory System](#agent-memory-system) pour la conception complète.

#### Warn a user

Envoie un message privé d’avertissement à un utilisateur concernant un commentaire spécifique, et enregistre atomiquement l’avertissement dans la mémoire de l’agent. La politique d’escalade de la plateforme est construite autour de cet outil : avertir d’abord, bannir seulement en cas de récidive. Moins souvent soumis à approbation que `ban_user`, mais songez à le soumettre pendant les premières semaines de vie d’un agent. Voir [Warn user](#tool-warn-user) pour la page complète.

#### Ban a user

L’outil le plus conséquent qu’un agent puisse appeler. Bannie un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant aussi l’IP, éventuellement en supprimant tous les commentaires de l’utilisateur. Les deux options destructrices (IP, suppression-tous) sont verrouillées derrière des opt-ins supplémentaires sur le formulaire d’édition. Dans la région UE, tous les bannissements requièrent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Envisagez fortement de le soumettre à approbation partout. Voir [Ban user](#tool-ban-user) pour la page complète.

### Options secondaires de l'outil Ban

L'outil Ban expose deux options destructrices - `delete-all-comments` et `ban-by-IP` - qui sont totalement invisibles au modèle tant que vous ne les avez pas activées via la section **Ban options** sur le formulaire d'édition. Même si le modèle invente le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Ban user](#tool-ban-user).