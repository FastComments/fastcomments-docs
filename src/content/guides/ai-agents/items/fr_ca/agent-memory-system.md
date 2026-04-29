Agent memory est un pool de paires clé-valeur **partagé** au niveau du locataire, que chaque agent de votre locataire peut lire et écrire. Il existe pour permettre aux agents de conserver le contexte entre les exécutions.

### Pourquoi la mémoire existe

Le contexte du LLM est par exécution. Sans mémoire, un agent qui signale un avertissement à un utilisateur n'a aucun moyen de se souvenir de cet avertissement la prochaine fois qu'il rencontre le même utilisateur. La politique d'escalade de la plateforme — « avertir avant de bannir » — dépend de la capacité de l'agent à retrouver l'avertissement précédent. La mémoire est ce qui rend cela possible.

### Deux types de mémoire

- **AVERTISSEMENT** - écrit automatiquement dans le cadre du flux [`warn_user`](#tool-warn-user). L'agent n'écrit pas les enregistrements **AVERTISSEMENT** manuellement ; ils sont un effet secondaire du fait d'avertir un utilisateur.
- **NOTE** - écrite par [`save_memory`](#tools-overview). Contexte général que l'agent veut que les agents futurs connaissent.

La politique d'escalade recherche spécifiquement les enregistrements `WARNING` lorsqu'elle décide si un bannissement est justifié.

### À l'échelle du locataire, partagé entre agents

Tous les agents de votre locataire partagent **un seul pool de mémoire**. Une note sauvegardée par l'Agent A est visible par les appels `search_memory` de l'Agent B. C'est intentionnel — vous voulez que les notes d'un agent de triage informent les décisions d'un agent modérateur.

`tenantId` est défini par l'exécuteur à partir du locataire de l'agent lui‑même — jamais à partir des arguments du LLM — donc les fuites de mémoire entre locataires sont impossibles par construction.

### Ce que contient un enregistrement de mémoire

Chaque entrée de mémoire contient :

- **Quel agent l'a écrite**, et quand.
- **De qui il s'agit** — l'utilisateur que cette mémoire décrit. L'agent ne peut pas fabriquer cela ; la plateforme le remplit automatiquement à partir de ce qui a déclenché l'agent.
- **Un signal caché de compte alternatif** — la plateforme enregistre également (privément) l'empreinte IP du commentaire d'origine, de sorte que les recherches de mémoire futures peuvent faire remonter des notes concernant d'autres comptes publiant depuis la même IP. L'empreinte n'est jamais montrée à l'agent ni au LLM.
- **La note elle‑même** — jusqu'à 2000 caractères de texte libre.
- **Des balises** pour la récupération — jusqu'à 10 courtes balises.
- **Un type** — soit un avertissement, soit une note générale.
- **Un lien de commentaire optionnel** — si la mémoire est liée à un commentaire spécifique.

### Comportement de recherche

[`search_memory`](#tools-overview) renvoie jusqu'à 25 enregistrements, triés du plus récent au plus ancien, portés automatiquement à (l'utilisateur ayant déclenché) OU (les autres comptes sur l'IP du déclencheur). Les résultats sont également limités à 8000 caractères au total pour l'ensemble du contenu retourné — les entrées plus anciennes sont exclues si le plafond est atteint.

L'agent ne transmet pas `userId` ni `targetIpHash`. Les deux sont définis par l'exécuteur.

### Persistance

La mémoire n'a **pas de TTL**. Les enregistrements persistent jusqu'à suppression explicite. Les enregistrements AVERTISSEMENT concernant un utilisateur ne sont volontairement jamais supprimés automatiquement — l'historique d'escalade doit être retrouvable indéfiniment, sinon la vérification « rechercher avant de bannir » de la plateforme n'a aucun sens.

Les trois façons dont la mémoire est supprimée :

- Un modérateur supprime le commentaire sous‑jacent — toute mémoire liée à ce commentaire est supprimée en cascade.
- Un utilisateur est supprimé — toutes les entrées de mémoire concernant cet utilisateur sont supprimées dans la même transaction.
- Votre locataire est supprimé.

Il n'existe aujourd'hui aucune interface d'administration pour supprimer des enregistrements de mémoire individuellement.

### Mémoire en mode dry-run

Les agents en mode dry-run n'écrivent **pas** de mémoire. C'est intentionnel : les décisions hypothétiques d'un agent en dry-run ne doivent pas polluer le pool de mémoire partagé. La lecture via `search_memory` fonctionne normalement en dry-run — l'agent peut voir de vraies mémoires d'agents en production — il ne peut simplement pas y ajouter.

### Mémoire lors des replays

Comme pour le dry-run : les agents de replay n'écrivent pas de mémoire. Les replays sont uniquement en aperçu. Voir [Exécutions de test (Replays)](#test-runs-replays).

### Résumé des contraintes

| Limit | Value |
|---|---|
| Memory content max length | 2000 caractères |
| Memory tag max length | 64 caractères |
| Memory tags max count | 10 |
| Memory query max length | 200 caractères |
| Memory search result limit | 25 enregistrements |
| Memory search total content cap | 8000 caractères |

### Voir aussi

- [Outil : save_memory](#tools-overview) pour l'écriture.
- [Outil : search_memory](#tools-overview) pour la lecture.
- [Outil : warn_user](#tool-warn-user) — le seul outil qui écrit de la mémoire de type WARNING.
- [Outil : ban_user](#tool-ban-user) — le prompt système exige que `search_memory` soit appelé avant celui‑ci.