L'outil de bannissement est l'action la plus conséquente qu'un agent puisse appeler. Il bannit un utilisateur de votre communauté, pour une durée fixe et avec quelques options.

### Ce que ça fait

L'agent choisit l'une des six durées :

- Une heure
- Un jour
- Une semaine
- Un mois
- Six mois
- Un an

Il choisit également entre un **bannissement visible** (l'utilisateur voit un message clair de bannissement et peut faire appel) et un **shadow ban** (l'utilisateur peut continuer à publier mais son contenu est masqué aux autres utilisateurs). Les instructions de la plateforme indiquent à l'agent de privilégier les bannissements visibles pour les cas de première infraction ou limites, et les shadow bans pour les récidivistes manifestement malveillants.

### Les deux sous-options destructrices

Deux options supplémentaires sont **masquées par défaut à l'agent**. Pour activer l'une ou l'autre, cochez la case correspondante dans la section **Options de bannissement** du formulaire d'édition de l'agent :

- **Autoriser la suppression de tous les commentaires de l'utilisateur.** Lorsqu'elle est activée, l'agent peut choisir de supprimer également chaque commentaire que l'utilisateur banni a jamais publié dans votre tenant. À réserver pour le spam manifeste, le doxxing ou les abus coordonnés où le contenu existant n'a aucune valeur. **Destructif et irréversible.**
- **Autoriser le bannissement par IP.** Lorsqu'elle est activée, l'agent peut choisir de bannir également l'IP depuis laquelle le commentaire a été posté. Utile contre l'évasion de bannissement par comptes alternatifs. **À éviter pour les adresses IP partagées** (entreprises, écoles, opérateurs mobiles) — des utilisateurs innocents sur le même réseau seront bloqués.

La plateforme applique également ces restrictions côté serveur : même si l'agent se dérègle et essaie d'invoquer l'option, la requête est refusée à moins que vous n'ayez activé cette option.

### Politique d'escalade

Avant de bannir, la plateforme demande à l'agent de :

1. Rechercher dans la [mémoire de l'agent](#agent-memory-system) d'éventuels avertissements ou notes concernant l'utilisateur.
2. Privilégier l'[avertissement](#tool-warn-user) plutôt que le bannissement pour les premières infractions.
3. Ne sauter l'étape d'avertissement que pour les cas clairement flagrants (contenu illégal, doxxing, spam coordonné) — et expliquer pourquoi dans sa justification.

Cette politique figure dans les instructions de l'agent, ce n'est pas une règle stricte côté serveur, c'est pourquoi il est fortement recommandé de **soumettre les bannissements à approbation**.

### Région UE : approbation humaine requise

Dans la région UE, cet outil est **verrouillé en attente d'approbation** en vertu de l'article 17 du Digital Services Act. Chaque bannissement effectué par un agent sur un tenant de la région UE arrive dans la [boîte de réception des approbations](#approval-workflow) pour examen humain. Voir [Conformité à l'article 17 du DSA pour l'UE](#eu-dsa-compliance).

### Recommandations

- Soumettez à approbation partout pendant au moins le premier mois.
- Verrouillez toujours l'option **supprimer-tous-les-commentaires** si vous l'activez — elle est irréversible.
- Envisagez de verrouiller l'option **bannissement IP** même après que l'agent ait gagné en confiance — le coût d'un bannissement IP sur un réseau partagé n'apparaît pas dans l'historique d'exécution de l'agent.

### Voir aussi

- [Bannir des utilisateurs](/guide-moderation.html#banning-users) et [Bannir des utilisateurs avec des caractères génériques](/guide-moderation.html#banning-users-wildcards) dans le guide de modération pour comprendre comment les bannissements fonctionnent à l'échelle de la plateforme.
- [Avertir l'utilisateur](#tool-warn-user) — l'étape d'escalade la plus douce.