Un agent a des **outils** qui correspondent aux actions qu'il peut entreprendre. Le formulaire d'édition de l'agent comporte une section **Appels d'outil autorisés** où vous cochez les outils que cet agent est autorisé à utiliser, et une section **Approbations** où vous cochez les actions qui doivent être approuvées par un humain avant d'entrer en vigueur.

Il existe trois niveaux pour tout outil :

- **Interdit** - l'agent ne peut pas le voir ni l'utiliser.
- **Autorisé, sans approbation** - l'agent l'utilise directement. Enregistré dans l'historique d'exécution.
- **Autorisé, avec approbation** - l'appel de l'agent est mis en file d'attente pour révision humaine et ne s'exécute que lorsqu'un humain l'approuve.

Les outils interdits sont silencieux : l'agent ne peut pas en faire la demande et la plateforme les refuse catégoriquement. Les outils soumis à approbation passent toujours par la [boîte de réception des approbations](#approval-workflow).

### Piste d'audit pour chaque action

Chaque action effectuée par l'agent est enregistrée avec une courte justification (1-2 phrases expliquant pourquoi) et un score de confiance (0.0-1.0). Les deux apparaissent dans la [Vue détaillée d'exécution](#run-detail-view) et sur chaque [approbation](#approval-workflow). La recherche dans la mémoire est la seule exception en lecture seule : elle n'est pas enregistrée comme action et est toujours disponible indépendamment de la liste d'autorisation.

### Référence des outils

#### Poster des commentaires

Permet à l'agent de publier un commentaire en son nom. Le commentaire est affiché publiquement sous le nom d'affichage de l'agent. Utilisé par les agents d'accueil et de résumé. Réversible - tout modérateur peut supprimer un mauvais commentaire. Soumettez-le à approbation si votre communauté exige que chaque message public soit revu par un humain.

#### Modifier un commentaire

Permet à l'agent de réécrire le texte d'un commentaire dans le périmètre autorisé. Le texte original est conservé dans le journal d'audit du commentaire. À réserver aux cas limités - effacer des informations personnelles divulguées par un utilisateur, ou amender la propre réponse antérieure de l'agent. Ne sert pas à réécrire des opinions ou à adoucir le ton. Voir [Modifier un commentaire](#tool-edit-comment) pour la page complète.

#### Voter sur des commentaires

Permet à l'agent de voter pour ou contre un commentaire. Le vote compte dans le total comme n'importe quel autre vote. La plupart des communautés préfèrent que les bots ne votent pas ; non activé dans aucun modèle de démarrage. Si vous l'autorisez, le vote est réversible.

#### Épingler / désépingler un commentaire

Permet à l'agent d'épingler un commentaire en haut de la page ou d'en désépingler un déjà épinglé. La plateforme n'impose pas de règle d'une épingle par fil de discussion, donc un agent chargé d'épingler doit être instruit de désépingler d'abord le commentaire épinglé précédent. Pour découvrir ce qui est déjà épinglé sur la même page, l'agent peut appeler l'outil en lecture seule `get_pinned_comments` (voir ci-dessous). Utilisé par le modèle Top Comment Pinner.

#### Verrouiller / déverrouiller un commentaire

Permet à l'agent d'empêcher de nouvelles réponses sous un commentaire, ou de rétablir la possibilité de répondre. Le commentaire verrouillé reste visible. Utile pour calmer des fils enflammés, associé à un déverrouillage différé. Pour découvrir ce qui est actuellement verrouillé sur la même page, l'agent peut appeler l'outil en lecture seule `get_locked_comments` (voir ci-dessous).

#### Marquer / démarquer comme spam

Permet à l'agent de marquer un commentaire comme spam (le cachant aux lecteurs et alimentant le classificateur de spam) ou de lever ce drapeau. L'outil de base pour tout agent de modération. Réversible.

#### Approuver / désapprouver un commentaire

Permet à l'agent d'afficher un commentaire en attente aux lecteurs, ou de masquer un commentaire déjà visible. Le plus utile pour les tenants qui retiennent les nouveaux commentaires pour examen par un modérateur.

#### Marquer un commentaire comme examiné

Un outil d'état de file : marque un commentaire comme « un modérateur (ou un agent) l'a examiné ». Ne change pas la visibilité. Peu risqué ; rarement soumis à approbation.

#### Attribuer un badge

Permet à l'agent d'attribuer à un utilisateur un badge que vous avez configuré pour votre locataire. Réversible par un modérateur. Lorsque cet outil est activé, l'agent peut voir les badges de votre locataire et choisir le bon lui-même, de sorte que vous n'avez pas besoin de coller les identifiants de badge dans vos directives communautaires ou votre prompt initial. Pour orienter quel badge est attribué pour quel comportement, référez-vous aux badges par leur **Libellé d'affichage** dans le prompt.

#### Envoyer un courriel

Permet à l'agent d'envoyer un courriel en texte brut à l'auteur d'un commentaire dans le périmètre du déclencheur. L'agent ne voit jamais l'adresse courriel du destinataire - il choisit un commentaire et la plateforme livre au courriel que cet auteur a laissé lors de la publication. L'adresse d'expédition est l'expéditeur de marque de votre locataire (avec DKIM) lorsque le domaine du commentaire correspond à un domaine configuré, sinon le défaut de la plateforme. À utiliser avec parcimonie - le courriel est l'outil le plus soumis aux frictions et les mauvais courriels sont difficiles à annuler.

#### Enregistrer / rechercher dans la mémoire de l'agent

Deux outils jumelés qui lisent et écrivent dans un pool de notes partagé concernant l'utilisateur pour lequel un déclencheur s'est produit. La mémoire est partagée entre tous les agents de votre locataire, donc les notes d'un agent de triage informent les décisions d'un agent modérateur. La recherche est en lecture seule et toujours disponible ; l'enregistrement est rarement soumis à approbation. Voir [Système de mémoire des agents](#agent-memory-system) pour la conception complète.

#### Obtenir les commentaires épinglés / Obtenir les commentaires verrouillés

Deux outils de découverte en lecture seule qui listent les commentaires épinglés (ou verrouillés) sur la même page (`urlId`) sur laquelle le déclencheur s'est produit. Ils ne prennent aucun argument - la page est lue depuis le contexte du déclencheur, donc l'agent ne peut pas pivoter vers d'autres pages. Utilisez-les lorsqu'un agent doit agir sur un commentaire déjà épinglé ou verrouillé - typiquement l'appel initial avant `unpin_comment` ou `unlock_comment`, ou avant d'épingler un nouveau commentaire afin que l'existant puisse être désépinglé d'abord.

Chaque outil est autorisé séparément dans **Appels d'outil autorisés** (l'administrateur coche `List pinned comments on the current page` ou `List locked comments on the current page`). Ils ne peuvent pas être soumis à approbation - les outils en lecture seule n'ont aucun effet secondaire à approuver. Les appeler n'est pas enregistré comme une action dans l'historique d'exécution ; seul l'appel résultant `unpin_comment` / `unlock_comment` / `pin_comment` (le cas échéant) apparaît. La liste est limitée aux 20 correspondances les plus récentes par appel.

Important à comprendre : lorsqu'un de ces outils renvoie un commentId, ce commentId est ajouté au périmètre par exécution de l'agent, donc l'appel de suivi `unpin_comment` / `unlock_comment` est validé par la vérification de sécurité ciblée de la plateforme. Sans avoir d'abord appelé l'outil de découverte, l'agent ne peut pas agir sur des commentaires qui ne sont pas déjà dans le périmètre immédiat du déclencheur. Ainsi, un agent de type désépinglage dispose typiquement des deux outils activés (par ex. `get_pinned_comments` plus `unpin_comment`).

#### Avertir un utilisateur

Envoie un message privé (DM) d'avertissement à un utilisateur au sujet d'un commentaire spécifique, et enregistre atomiquement l'avertissement dans la mémoire de l'agent. La politique d'escalade de la plateforme est construite autour de cet outil - avertir d'abord, bannir uniquement si l'utilisateur récidive. Voir [Avertir l'utilisateur](#tool-warn-user) pour la page complète.

#### Bannir un utilisateur

L'outil le plus conséquent qu'un agent puisse appeler. Banni un utilisateur pour une durée fixe, éventuellement en shadow ban, éventuellement en bannissant aussi l'IP, éventuellement en supprimant tous les commentaires de l'utilisateur. Les deux options destructrices (IP, suppression-tous) sont soumises à des opt-ins supplémentaires sur le formulaire d'édition. Dans la région UE, tous les bannissements requièrent une approbation humaine (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)). Voir [Bannir un utilisateur](#tool-ban-user) pour la page complète.

### Sous-options de l'outil de bannissement

L'outil de bannissement expose deux options destructrices - delete-all-comments et ban-by-IP - qui sont cachées au modèle jusqu'à ce que vous les activiez via la section **Options de bannissement** sur le formulaire d'édition. Même si le modèle hallucine le paramètre, la plateforme refuse les valeurs que vous n'avez pas activées. Voir [Bannir un utilisateur](#tool-ban-user).