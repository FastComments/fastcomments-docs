Les **outils** d'un agent sont les actions qu'il peut effectuer. Le formulaire d'édition de l'agent comporte une section **Appels d'outils autorisés** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approbations** où vous cochez les actions qui devraient nécessiter l'approbation d'un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Interdit** - l'agent ne peut pas le voir ni l'utiliser.
- **Autorisé, sans approbation** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Autorisé, avec approbation** - l'appel de l'agent est mis en file d'attente pour révision humaine et n'est exécuté que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas les demander et la plateforme les refuse purement et simplement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Journal d'audit pour chaque action

Chaque action effectuée par l'agent est enregistrée avec une brève justification (1 à 2 phrases expliquant pourquoi) et un score de confiance (0.0-1.0). Les deux apparaissent dans la [Vue du détail d'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et est toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Publier des commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible : n'importe quel modérateur peut supprimer un mauvais commentaire. Généralement autorisé sans approbation ; soumettez-le à approbation si votre communauté exige que chaque message public soit revu par un humain.

#### Voter sur des commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total de votes du commentaire comme n'importe quel autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou d'en retirer l'épingle s'il est déjà épinglé. La plateforme n'applique pas de règle d'une épingle par fil, donc un agent chargé d'épingler devrait d'abord retirer l'épingle du commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher d'autres réponses sous un commentaire, ou de restaurer les réponses. Le commentaire verrouillé reste visible. Utile pour calmer les fils enflammés, à associer à un déverrouillage différé. Réversible mais visible par votre communauté ; envisagez de le soumettre à approbation dans les communautés sensibles.

#### Marquer / annuler le marquage comme spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou de supprimer ce signalement. L'outil de base pour tout agent de modération. Réversible. Envisagez fortement de le soumettre à approbation pendant les premières semaines pendant que vous développez la confiance envers l'agent.

#### Approuver / désapprouver un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les locataires qui retiennent les nouveaux commentaires pour révision par un modérateur. Risqué lorsqu'on désapprouve un commentaire visible - envisagez de le soumettre à approbation.

#### Marquer un commentaire comme examiné

Outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) l'a examiné ». Ne change pas la visibilité. Faible risque ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer un badge à un utilisateur à partir de la configuration de badges de votre locataire. Réversible par un modérateur. Rarement soumis à approbation. L'agent doit connaître l'ID du badge, donc incluez les ID pertinents dans vos [lignes directrices communautaires](#community-guidelines) ou votre [invite initiale](#personality-prompt).

#### Envoyer un courriel

Permet à l'agent d'envoyer un courriel en texte brut depuis `noreply@fastcomments.com` vers une adresse qu'il choisit. À utiliser avec parcimonie : le courriel est l'outil le plus contraignant et les courriels problématiques sont difficiles à corriger. Envisagez fortement de le soumettre à approbation, et redirigez les courriels d'approbation vers la personne qui possède la boîte de réception que l'agent finira par contacter.

#### Enregistrer / rechercher la mémoire de l'agent

Deux outils appariés qui lisent et écrivent dans un pool de notes partagé à propos de l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre locataire, donc les notes d'un agent de tri informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Système de mémoire des agents](#agent-memory-system) pour la conception complète.

#### Avertir un utilisateur

Envoie un avertissement privé en DM à un utilisateur concernant un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil : avertir d'abord, bannir uniquement si l'utilisateur récidive. Moins souvent soumis à approbation que `ban_user`, mais envisagez de le soumettre à approbation pendant les premières semaines de vie d'un agent. Voir [Avertir l'utilisateur](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus conséquent qu'un agent puisse appeler. Banni un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant aussi l'IP, éventuellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, suppression de tous les commentaires) sont soumises à des opt-ins supplémentaires sur le formulaire d'édition. Dans la région de l'UE, tous les bannissements nécessitent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Envisagez fortement de le soumettre à approbation partout. Voir [Bannir un utilisateur](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil Ban expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont totalement invisibles pour le modèle tant que vous ne les avez pas activées via la section **Ban options** du formulaire d'édition. Même si le modèle hallucinait le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Bannir un utilisateur](#tool-ban-user).