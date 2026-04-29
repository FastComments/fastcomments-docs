---
Les **outils** d'un agent sont les actions qu'il peut entreprendre. Le formulaire d'édition de l'agent comporte une section **Appels d'outils autorisés** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approbations** où vous cochez les actions qui doivent être approuvées par un humain avant de prendre effet.

Il existe trois niveaux pour chaque outil :

- **Interdit** - l'agent ne peut pas le voir ni l'utiliser.
- **Autorisé, sans approbation** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Autorisé, avec approbation** - l'appel de l'agent est mis en file d'attente pour examen humain et n'est exécuté que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas en faire la demande et la plateforme les refuse purement et simplement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit sur chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1–2 phrases expliquant pourquoi) et un score de confiance (0.0–1.0). Les deux apparaissent dans la [Run Detail View](#run-detail-view) et sur chaque [approval](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme une action et est toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Publier des commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom affiché de l'agent. Utilisé par les agents d'accueil et de synthèse. Réversible : tout modérateur peut supprimer un mauvais commentaire. Généralement autorisé sans approbation ; limitez-le si votre communauté exige que chaque message public soit relu par un humain.

#### Modifier un commentaire

Permet à l'agent de réécrire le texte d'un commentaire concerné. Le texte original est conservé dans le journal d'audit du commentaire. À réserver à des cas restreints : suppression de PII divulguées par un utilisateur, ou correction de la propre réponse antérieure de l'agent. À ne pas utiliser pour réécrire des opinions ou adoucir un ton. **Envisagez fortement de le soumettre à approbation.** Voir [Edit comment](#tool-edit-comment) pour la page complète.

#### Voter sur des commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total de votes du commentaire comme tout autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou de désépingler un commentaire déjà épinglé. La plateforme n'impose pas de règle d'une épingle par fil, donc un agent chargé d'épingler doit être instruit de désépingler d'abord le commentaire précédemment épinglé. Utilisé par le modèle Top Comment Pinner. Réversible ; généralement autorisé sans approbation.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher de nouvelles réponses sous un commentaire, ou de restaurer les réponses. Le commentaire verrouillé reste visible. Utile pour des périodes de refroidissement sur des fils enflammés, associé à un déverrouillage différé. Réversible mais visible par votre communauté ; envisagez de le soumettre à approbation dans les communautés à enjeux élevés.

#### Marquer / retirer le statut de spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou d'effacer ce drapeau. L'outil essentiel pour tout agent de modération. Réversible. Envisagez fortement de le soumettre à approbation pendant les premières semaines le temps de bâtir la confiance dans l'agent.

#### Approuver / retirer l'approbation d'un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Très utile pour les tenants qui retiennent les nouveaux commentaires pour relecture par un modérateur. Risqué quand on retire l'approbation d'un commentaire visible — envisagez de le soumettre à approbation.

#### Marquer un commentaire comme examiné

Outil d'état de file : marque un commentaire comme « un modérateur (ou agent) l'a examiné ». Ne change pas la visibilité. Faible risque ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer à un utilisateur un badge depuis la configuration de badges de votre tenant. Réversible par un modérateur. Rarement soumis à approbation. L'agent doit connaître l'ID du badge, donc incluez les ID pertinents dans vos [community guidelines](#community-guidelines) ou dans votre [initial prompt](#personality-prompt).

#### Envoyer un e-mail

Permet à l'agent d'envoyer un e-mail en texte brut depuis `noreply@fastcomments.com` vers une adresse qu'il choisit. À utiliser avec parcimonie : l'e-mail est l'outil le plus coûteux en termes de friction et les mauvais e-mails sont difficiles à annuler. Envisagez fortement de le soumettre à approbation, et redirigez les demandes d'approbation d'e-mails vers la personne qui gère la boîte de réception que l'agent aura tendance à contacter.

#### Enregistrer / rechercher la mémoire de l'agent

Deux outils appariés qui lisent et écrivent un pool de notes partagé sur l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre tenant, ainsi les notes d'un agent de triage informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Agent Memory System](#agent-memory-system) pour la conception complète.

#### Avertir un utilisateur

Envoie un message privé d'avertissement à un utilisateur au sujet d'un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil : avertir d'abord, bannir seulement si l'utilisateur récidive. Moins souvent soumis à approbation que `ban_user`, mais envisagez de le soumettre pendant les premières semaines de vie d'un agent. Voir [Warn user](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus conséquent qu'un agent puisse appeler. Banni un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant aussi l'IP, éventuellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, suppression-tout) sont soumises à des consentements supplémentaires sur le formulaire d'édition. Dans la région UE, tous les bannissements nécessitent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Envisagez fortement de le soumettre à approbation partout. Voir [Ban user](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil Ban expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont entièrement cachées au modèle tant que vous ne les avez pas activées via la section **Ban options** sur le formulaire d'édition. Même si le modèle invente le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Ban user](#tool-ban-user).

---