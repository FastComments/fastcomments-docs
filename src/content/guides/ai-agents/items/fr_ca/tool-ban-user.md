L'outil de bannissement est l'action la plus lourde de conséquences qu'un agent puisse appeler. Il bannit un utilisateur de votre communauté, pour une durée fixe et avec quelques options.

### Ce que ça fait

L'agent choisit l'une des six durées :

- Une heure
- Un jour
- Une semaine
- Un mois
- Six mois
- Un an

Il choisit aussi entre un **bannissement visible** (l'utilisateur voit un message de bannissement clair et peut faire appel) et un **bannissement discret** (l'utilisateur peut continuer à poster mais son contenu est caché aux autres utilisateurs). Les instructions de la plateforme demandent à l'agent de privilégier les bannissements visibles pour les cas de première infraction ou limites, et les bannissements discrets pour les récidivistes manifestement malveillants.

### Les deux sous-options destructrices

Deux options supplémentaires sont **cachées à l'agent par défaut**. Pour activer l'une ou l'autre, cochez la case correspondante dans la section **Options de bannissement** du formulaire d'édition de l'agent :

- **Allow deleting all of the user's comments.** Lorsqu'activée, l'agent peut choisir de supprimer aussi chaque commentaire que l'utilisateur banni a jamais publié dans votre tenant. Réserver aux spams évidents, doxxing ou abus coordonnés où le contenu existant n'a aucune valeur. **Destructeur et irréversible.**
- **Allow banning by IP.** Lorsqu'activée, l'agent peut choisir de bannir aussi l'IP depuis laquelle le commentaire a été posté. Utile contre l'évasion de bannissement par comptes alternatifs. **Éviter pour les IP partagées** (entreprise, école, opérateurs mobiles) — des utilisateurs innocents sur le même réseau seront bloqués.

La plateforme applique également ces restrictions côté serveur : même si l'agent déraille et tente d'invoquer l'option, la requête est refusée à moins que vous ayez choisi d'y adhérer.

### Politique d'escalade

Avant de bannir, la plateforme demande à l'agent de :

1. Rechercher dans la [mémoire de l'agent](#agent-memory-system) les avertissements ou notes antérieurs concernant l'utilisateur.
2. Privilégier un [avertissement](#tool-warn-user) plutôt que le bannissement pour une première infraction.
3. Ne passer l'étape d'avertissement que pour les cas manifestement graves (contenu illégal, doxxing, spams coordonnés) — et expliquer pourquoi dans sa justification.

Cette politique figure dans les instructions de l'agent, ce n'est pas une règle stricte côté serveur, c'est pourquoi il est fortement recommandé de **soumettre les bannissements à une approbation**.

### Région UE : approbation humaine requise

Dans la région UE, cet outil est **configuré pour exiger une approbation** en vertu de l'article 17 du Digital Services Act. Chaque bannissement provenant de n'importe quel agent sur un tenant situé dans la région UE atterrit dans la [boîte d'approbations](#approval-workflow) pour examen humain. Voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance).

### Recommandations

- Soumettre à approbation partout pendant au moins le premier mois.
- Toujours soumettre l'option **delete-all-comments** à approbation si vous l'activez — elle est irréversible.
- Envisager de soumettre également l'option **IP ban** à approbation même après que l'agent ait gagné la confiance — le coût d'un bannissement par IP sur un réseau partagé n'apparaît pas dans l'historique d'exécution de l'agent.

### Voir aussi

- [Bannir des utilisateurs](/guide-moderation.html#banning-users) et [Bannir des utilisateurs avec des jokers](/guide-moderation.html#banning-users-wildcards) dans le guide de modération pour comprendre comment les bannissements fonctionnent à l'échelle de la plateforme.
- [Avertir l'utilisateur](#tool-warn-user) — l'étape d'escalade plus douce.