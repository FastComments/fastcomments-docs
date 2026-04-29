La section **Contexte** du formulaire d’édition contrôle la quantité d’informations que l’agent reçoit à chaque exécution. Plus de contexte produit de meilleures décisions mais augmente le coût en tokens par exécution, donc vous ne voulez inclure que ce dont l’agent a réellement besoin.

### Ce qui est toujours inclus

Même si toutes les cases sont décochées, le message de contexte de l’agent inclut :

- Le **type d’événement déclencheur** (p. ex. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L’URL de la page et l’URL ID (lorsque connu).
- Le **commentaire** qui a déclenché l’exécution, s’il y en a un - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. L’email de l’auteur n’est **jamais** envoyé au LLM provider (minimisation des PII).
- Le **texte du commentaire précédent** pour les déclencheurs `COMMENT_EDIT` (pour que l’agent puisse comparer avant/après).
- La **direction du vote** pour les déclencheurs `COMMENT_VOTE_THRESHOLD`.
- Le triggering user ID et badge ID (pour les déclencheurs de badge de modérateur).

Tout texte non fiable - corps de commentaire, noms d’auteurs, titres de page, le document des directives lui‑même - est **encadré** dans le message de contexte avec des marqueurs comme `<<<COMMENT_TEXT>>> ... <<<END>>>`. Le prompt système de la plateforme demande au modèle de ne jamais suivre les instructions à l’intérieur de ces encadrements. C’est la défense contre l’injection de prompt de la plateforme ; vous n’avez pas besoin de la répéter dans votre prompt.

### Les trois cases à cocher

#### Inclure le commentaire parent et les réponses antérieures dans le même fil

Ajoute :
- Le **parent comment** - ID, author, text.
- **Sibling replies** - les réponses antérieures au même parent dans le même fil.

Utile pour : tout agent qui répond à un commentaire dans son contexte (accueillants, synthétiseurs de fil, modérateurs lisant les réponses dans une conversation).

Coût : faible à moyen. Borné par le nombre de siblings existant dans un fil donné.

#### Inclure le facteur de confiance du commentateur, l’âge du compte, l’historique de bannissements et les commentaires récents

Ajoute le bloc **AUTHOR_HISTORY** :

- **Account age in days** depuis l’inscription.
- **Trust factor (0-100)** - le score FastComments qui résume à quel point l’utilisateur est fiable sur ce site. Voir la page [Détection du spam](/guide-moderation.html#spam-detection) dans le guide de modération.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - si l’utilisateur a posté un texte identique récemment (signal anti-spam).
- **Same-IP cross-account signal** - nombre de commentaires depuis la même IP sous d’autres comptes (signal de comptes alternatifs). Le IP hash lui‑même n’est jamais envoyé au LLM.
- **Recent comments** - jusqu’à 5 des commentaires les plus récents de l’utilisateur, chacun tronqué à 300 caractères, encadrés comme texte non fiable.

Utile pour : tout agent de modération. Sans cela, le modèle bannit les nouveaux comptes et les utilisateurs de bonne foi de longue date avec la même posture.

Coût : moyen. Les commentaires récents sont ceux qui ajoutent le plus de tokens.

#### Inclure le titre de la page, le sous-titre, la description et les balises meta

Ajoute le bloc **PAGE_CONTEXT** - title, subtitle, description, et toutes les balises meta que FastComments a capturées pour la page.

Utile pour : les accueillants et les synthétiseurs de fil, où connaître le sujet de la page améliore substantiellement la qualité de la sortie.

Coût : faible.

### Directives communautaires

Le quatrième champ, **Directives communautaires**, est un bloc de politique en texte libre inclus dans le message de contexte du rôle utilisateur à chaque exécution, encadré comme texte non fiable de la même manière que les corps de commentaire et tout autre contenu fourni par l’utilisateur. L’agent le lit comme un texte de politique mais la plateforme ne le traite pas comme une instruction système. Consultez [les directives communautaires](#community-guidelines) pour savoir quoi y mettre.

### Ajouter du contexte de façon sélective

Ces cases s’appliquent par agent, pas globalement. Un schéma courant :

- Accueillant : page context **on**, thread context **off**, user history **off**.
- Modérateur : thread context **off**, user history **on**, page context **off**.
- Synthétiseur de fil : thread context **on**, page context **on**, user history **off**.

Visez le contexte minimum dont un agent a besoin pour être correct dans les appels qu’il effectue réellement — le contexte supplémentaire coûte des tokens à chaque exécution, même lorsque l’agent ne l’utilise pas.