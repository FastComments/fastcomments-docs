Agent memory est un pool de paires clé-valeur au niveau du tenant, **partagé** que tous les agents de votre tenant peuvent lire et écrire. Il existe pour permettre aux agents de conserver le contexte entre les exécutions.

### Pourquoi la mémoire existe

Le contexte LLM est par exécution. Sans mémoire, un agent qui avertit un utilisateur n'a aucun moyen de se souvenir de cet avertissement la prochaine fois qu'il rencontre le même utilisateur. La politique d'escalade de la plateforme - "avertir avant de bannir" - dépend du fait que l'agent puisse retrouver l'avertissement précédent. La mémoire est ce qui rend cela possible.

### Deux types de mémoire

- **WARNING** - écrit automatiquement dans le cadre du flux [`warn_user`](#tool-warn-user). L'agent n'écrit pas manuellement des enregistrements `WARNING` ; ils sont un effet secondaire du fait d'avertir un utilisateur.
- **NOTE** - écrit par [`save_memory`](#tools-overview). Contexte général que l'agent souhaite rendre disponible aux agents futurs.

La politique d'escalade cherche spécifiquement des enregistrements de type `WARNING` pour décider si un bannissement est justifié.

### Niveau tenant, partagé entre agents

Tous les agents de votre tenant partagent **un seul pool de mémoire**. Une note enregistrée par l'Agent A est visible par les appels `search_memory` de l'Agent B. C'est voulu : vous voulez que les notes d'un agent de triage informent les décisions d'un agent modérateur.

`tenantId` est défini par l'exécuteur à partir du tenant de l'agent lui-même - jamais à partir des arguments LLM - ainsi les fuites de mémoire entre tenants sont impossibles par conception.

### Ce qu'il y a dans un enregistrement de mémoire

Chaque entrée de mémoire contient :

- **Quel agent l'a écrite**, et quand.
- **De qui elle parle** - l'utilisateur que cette mémoire décrit. L'agent ne peut pas inventer cela ; la plateforme le remplit automatiquement à partir de ce qui a déclenché l'agent.
- **Un signal caché d'alt-account** - la plateforme enregistre aussi (privément) l'empreinte IP du commentaire d'origine, afin que les recherches de mémoire futures puissent faire apparaître des notes concernant d'autres comptes postant depuis la même IP. L'empreinte n'est jamais montrée à l'agent ni au LLM.
- **La note elle-même** - jusqu'à 2000 caractères de texte libre.
- **Des tags** pour la récupération - jusqu'à 10 tags courts.
- **Un type** - soit un warning soit une note générale.
- **Un lien de commentaire optionnel** - si la mémoire est liée à un commentaire spécifique.

### Comportement de recherche

[`search_memory`](#tools-overview) retourne jusqu'à 25 enregistrements, triés du plus récent au plus ancien, automatiquement limités à (l'utilisateur déclencheur) OU (d'autres comptes sur l'IP du déclencheur). Les résultats sont également limités à 8000 caractères au total sur l'ensemble du contenu retourné - les entrées plus anciennes sont abandonnées si le plafond est atteint.

L'agent ne passe pas `userId` ni `targetIpHash`. Les deux sont définis par l'exécuteur.

### Persistance

La mémoire n'a **pas de TTL**. Les enregistrements persistent jusqu'à suppression explicite. Les enregistrements WARNING concernant un utilisateur ne sont volontairement jamais supprimés automatiquement - l'historique d'escalade doit être retrouvable indéfiniment sinon la vérification "rechercher avant de bannir" de la plateforme n'a pas de sens.

Les trois façons dont la mémoire est supprimée :

- Un modérateur supprime le commentaire sous-jacent - toute mémoire liée à ce commentaire est en cascade.
- Un utilisateur est supprimé - toutes les entrées de mémoire concernant cet utilisateur sont supprimées dans la même transaction.
- Votre tenant est supprimé.

Il n'existe pas aujourd'hui d'interface d'administration pour supprimer des enregistrements de mémoire individuellement.

### Mémoire en dry-run

Les agents en dry-run n'écrivent **pas** de mémoire. C'est intentionnel : les décisions hypothétiques d'un agent en dry-run ne doivent pas polluer le pool de mémoire partagé. La lecture via `search_memory` fonctionne normalement en dry-run - l'agent peut voir les mémoires réelles des agents en production - il ne peut simplement pas y ajouter.

### Mémoire en relectures

Comme en dry-run : les agents de relecture n'écrivent pas de mémoire. Les relectures sont uniquement des prévisualisations. Voir [Exécutions de test (Replays)](#test-runs-replays).

### Récapitulatif des contraintes

| Limite | Valeur |
|---|---|
| Longueur maximale du contenu de la mémoire | 2000 chars |
| Longueur maximale d'un tag de mémoire | 64 chars |
| Nombre maximal de tags de mémoire | 10 |
| Longueur maximale d'une requête de mémoire | 200 chars |
| Limite de résultats de recherche de mémoire | 25 records |
| Plafond total du contenu des résultats de recherche de mémoire | 8000 chars |

### Voir aussi

- [Outil : save_memory](#tools-overview) pour l'écriture.
- [Outil : search_memory](#tools-overview) pour la lecture.
- [Outil : warn_user](#tool-warn-user) - le seul outil qui écrit de la mémoire de type WARNING.
- [Outil : ban_user](#tool-ban-user) - le prompt système exige d'appeler `search_memory` avant celui-ci.