La section **Contexte** du formulaire d'édition contrôle la quantité d'informations que l'agent reçoit à chaque exécution. Plus de contexte produit de meilleures décisions mais augmente le coût en tokens par exécution, vous ne devez donc envoyer que ce dont l'agent a réellement besoin.

### Ce qui est toujours inclus

Même si toutes les cases sont décochées, le message de contexte de l'agent inclut :

- Le **type d'événement déclencheur** (par ex. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL de la page et l'ID de l'URL** (lorsque connus).
- Le **commentaire** qui a déclenché l'exécution, s'il y en a un — ID, ID utilisateur de l'auteur, nom affiché de l'auteur, texte du commentaire, nombre de votes, nombre de signalements, indicateurs spam/approved/reviewed, ID du parent. L'email de l'auteur n'est **jamais** envoyé au fournisseur de LLM (minimisation des PII).
- Le **texte du commentaire précédent** pour les déclencheurs `COMMENT_EDIT` (pour que l'agent puisse comparer avant/après).
- La **direction du vote** pour les déclencheurs `COMMENT_VOTE_THRESHOLD`.
- L'**ID utilisateur déclencheur** et l'**ID du badge** (pour les déclencheurs de badge de modérateur).

Tout texte non fiable — corps des commentaires, noms d'auteurs, titres de pages, le document de directives lui‑même — est **encadré** dans le message de contexte par des marqueurs tels que `<<<COMMENT_TEXT>>> ... <<<END>>>`. Le prompt système de la plateforme ordonne au modèle de ne jamais suivre les instructions situées à l'intérieur de ces encadrements. Il s'agit de la défense contre l'injection de prompt de la plateforme ; vous n'avez pas besoin de le répéter dans votre prompt.

### Les trois cases à cocher

#### Inclure le commentaire parent et les réponses précédentes dans le même fil

Ajoute :
- Le **commentaire parent** — ID, auteur, texte.
- **Réponses sœurs** — les réponses antérieures au même parent dans le même fil.

Utile pour : tout agent qui répond à un commentaire en contexte (agents d'accueil, résumeurs de fil, modérateurs lisant les réponses dans les conversations).

Coût : petit à moyen. Borné par le nombre de réponses sœurs existant dans un fil donné.

#### Inclure le facteur de confiance du commentateur, l'âge du compte, l'historique de bannissements et les commentaires récents

Ajoute le bloc **AUTHOR_HISTORY** :

- **Âge du compte en jours** depuis l'inscription.
- **Facteur de confiance (0-100)** — le score FastComments qui résume le niveau de confiance accordé à l'utilisateur sur ce site. Voir la page [Détection du spam](/guide-moderation.html#spam-detection) du guide de modération.
- **Nombre de bannissements antérieurs.**
- **Nombre total de commentaires sur ce site.**
- **Nombre de contenus dupliqués** — si l'utilisateur a publié du texte identique récemment (signal anti-spam).
- **Signal de comptes croisés sur la même IP** — nombre de commentaires depuis la même IP sous d'autres comptes (signal de comptes alternatifs). Le hash de l'IP lui‑même n'est jamais envoyé au LLM.
- **Commentaires récents** — jusqu'à 5 des commentaires les plus récents de l'utilisateur, chacun tronqué à 300 caractères, encadrés comme texte non fiable.

Utile pour : tout agent de modération. Sans cela, le modèle bannit les comptes récents et les utilisateurs de bonne foi de longue date ayant le même comportement.

Coût : moyen. Les commentaires récents ajoutent le plus de tokens.

#### Inclure le titre de la page, le sous-titre, la description et les balises méta

Ajoute le bloc **PAGE_CONTEXT** — titre, sous-titre, description et toutes les balises méta que FastComments a capturées pour la page.

Utile pour : les agents d'accueil et les résumeurs de fil, où connaître le sujet de la page améliore sensiblement la qualité de la sortie.

Coût : faible.

### Consignes communautaires

Le quatrième champ, **Consignes communautaires**, est un bloc de politique en texte libre inclus dans le message de contexte du rôle utilisateur à chaque exécution, encadré comme texte non fiable de la même manière que les corps de commentaires et autres contenus fournis par les utilisateurs. L'agent le lit comme un texte de politique mais la plateforme ne le traite pas comme une instruction système. Voir [Consignes communautaires](#community-guidelines) pour savoir quoi y mettre.

### Ajouter du contexte de manière sélective

Ces cases à cocher s'appliquent par agent, pas globalement. Un schéma courant :

- Agent d'accueil : contexte de la page **activé**, contexte du fil **désactivé**, historique utilisateur **désactivé**.
- Modérateur : contexte du fil **désactivé**, historique utilisateur **activé**, contexte de la page **désactivé**.
- Résumeur de fil : contexte du fil **activé**, contexte de la page **activé**, historique utilisateur **désactivé**.

Choisissez le minimum de contexte nécessaire pour que l'agent soit correct dans les appels qu'il effectue réellement — du contexte supplémentaire coûte des tokens à chaque exécution, même lorsque l'agent ne l'utilise pas.