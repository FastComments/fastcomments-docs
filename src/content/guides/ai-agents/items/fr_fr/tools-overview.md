Les **outils** d'un agent sont les actions qu'il peut effectuer. Le formulaire d'édition de l'agent comporte une section **Appels d'outils autorisés** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approbations** où vous cochez les actions qui doivent être approuvées par un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Interdit** - l'agent ne peut pas le voir ni l'utiliser.
- **Autorisé, sans approbation** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Autorisé, avec approbation** - l'appel de l'agent est mis en file d'attente pour révision humaine et ne s'exécute que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas les demander et la plateforme les refuse purement et simplement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit pour chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1 à 2 phrases expliquant pourquoi) et un score de confiance (0,0–1,0). Les deux apparaissent dans la [Vue détaillée d'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et est toujours disponible quelle que soit la liste d'autorisation.

### Référence des outils

#### Publication de commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible : tout modérateur peut supprimer un mauvais commentaire. Généralement autorisé sans approbation ; soumettez-le à approbation si votre communauté exige que chaque message public soit revu par un humain.

#### Modification d'un commentaire

Permet à l'agent de réécrire le texte d'un commentaire dans le périmètre autorisé. Le texte original est conservé dans le journal d'audit du commentaire. À réserver à des cas limités : expurger des PII qu'un utilisateur a divulguées, ou corriger la propre réponse antérieure de l'agent. Ne pas l'utiliser pour réécrire des opinions ou atténuer un ton. **Envisagez fortement de le soumettre à approbation.** Voir [Edit comment](#tool-edit-comment) pour la page complète.

#### Voter sur les commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total du commentaire comme tout autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou d'en désépingler un déjà épinglé. La plateforme n'applique pas de règle d'une seule épingle par fil, donc un agent chargé d'épingler doit être instruit de désépingler d'abord le commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher les réponses supplémentaires sous un commentaire, ou de rétablir les réponses. Le commentaire verrouillé reste visible. Utile pour calmer des fils enflammés, à associer à un déverrouillage différé. Réversible mais visible par votre communauté ; envisagez de le soumettre à approbation dans les communautés à fort enjeu.

#### Marquer / dé-marquer comme spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou de supprimer ce drapeau. L'outil de base pour tout agent de modération. Réversible. Envisagez fortement de le soumettre à approbation pendant les premières semaines le temps d'instaurer la confiance envers l'agent.

#### Approuver / désapprouver un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les locataires qui retiennent les nouveaux commentaires pour révision modératrice. Fortement risqué lors de la désapprobation d'un commentaire visible — envisagez de le soumettre à approbation.

#### Marquer un commentaire comme revu

Outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) a regardé ceci ». Ne change pas la visibilité. Faible enjeu ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer à un utilisateur un badge que vous avez configuré pour votre tenant. Réversible par un modérateur. Rarement soumis à approbation. Lorsque cet outil est activé, l'agent peut voir les badges de votre tenant et choisir le bon lui-même, vous n'avez donc pas besoin de coller les identifiants de badge dans vos directives communautaires ou votre prompt initial. Si vous souhaitez diriger quel badge est attribué pour quel comportement, référez-vous aux badges par leur **Libellé affiché** dans le prompt.

#### Envoyer un e-mail

Permet à l'agent d'envoyer un e-mail en texte brut à l'auteur d'un commentaire dans le périmètre du déclencheur. L'agent ne voit jamais l'adresse e-mail du destinataire : il choisit un commentaire et la plateforme délivre à l'adresse que l'auteur a fournie lors de sa publication. L'adresse de l'expéditeur est l'expéditeur de marque de votre tenant (avec DKIM) lorsque le domaine du commentaire correspond à un domaine configuré, sinon c'est le défaut de la plateforme. À utiliser avec parcimonie : l'e-mail est l'outil le plus à friction et les mauvais e-mails sont difficiles à annuler. Envisagez fortement de le soumettre à approbation, et orientez les e-mails d'approbation vers la personne qui gère la boîte de réception que l'agent finira par contacter.

#### Sauvegarder / rechercher dans la mémoire de l'agent

Deux outils appariés qui lisent et écrivent une piscine de notes partagée au sujet de l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre tenant, donc les notes d'un agent de triage informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; la sauvegarde est rarement soumise à approbation. Voir [Agent Memory System](#agent-memory-system) pour la conception complète.

#### Avertir un utilisateur

Envoie un DM privé avertissant un utilisateur à propos d'un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil : avertir d'abord, bannir seulement en cas de récidive. Moins souvent soumis à approbation que `ban_user`, mais envisagez de le soumettre à approbation pendant les premières semaines de vie d'un agent. Voir [Warn user](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus lourd de conséquences qu'un agent peut appeler. Bannie un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant l'IP, éventuellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, delete-all) sont soumises à des opt-ins supplémentaires sur le formulaire d'édition. Dans la région UE, tous les bannissements nécessitent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Envisagez fortement de le soumettre à approbation partout. Voir [Ban user](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil Ban expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont entièrement cachées au modèle tant que vous ne les avez pas activées via la section **Ban options** sur le formulaire d'édition. Même si le modèle hallucine le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Ban user](#tool-ban-user).