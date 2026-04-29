La section **Context** du formulaire d'édition contrôle la quantité d'informations que l'agent reçoit à chaque exécution. Plus de contexte produit de meilleures décisions mais augmente le coût en tokens par exécution, donc vous ne voulez que ce dont l'agent a réellement besoin.

### Ce qui est toujours inclus

Même si toutes les cases sont décochées, le message de contexte de l'agent inclut :

- Le **type d'événement déclencheur** (par ex. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL de la page et l'ID d'URL** (lorsque connu).
- Le **commentaire** qui a déclenché l'exécution, s'il y en a un - ID, ID utilisateur de l'auteur, nom d'affichage de l'auteur, texte du commentaire, nombres de votes, nombre de signalements, spam/approved/reviewed flags, ID du parent. L'adresse e-mail de l'auteur n'est **jamais** envoyée au fournisseur LLM (minimisation des PII).
- Le **texte du commentaire précédent** pour les déclencheurs `COMMENT_EDIT` (afin que l'agent puisse comparer avant/après).
- La **direction du vote** pour les déclencheurs `COMMENT_VOTE_THRESHOLD`.
- L'**ID de l'utilisateur déclencheur** et l'**ID du badge** (pour les déclencheurs de badge modérateur).
- Le **catalogue de badges** de votre tenant (nom, libellé d'affichage, description) lorsque l'agent est autorisé à attribuer des badges, afin que l'agent puisse en choisir un approprié sans que vous ayez à énumérer les badges dans l'invite.

Tout texte non fiable - corps de commentaires, noms d'auteurs, titres de page, le document de directives lui‑même - est **encadré** dans le message de contexte avec des marqueurs comme `<<<COMMENT_TEXT>>> ... <<<END>>>`. Le system prompt de la plateforme indique au modèle de ne jamais suivre les instructions à l'intérieur de ces encadrements. Il s'agit de la défense contre l'injection de prompt de la plateforme ; vous n'avez pas besoin de la répéter dans votre invite.

### Les trois cases à cocher

#### Inclure le commentaire parent et les réponses précédentes dans le même fil

Ajoute :
- Le **commentaire parent** - ID, auteur, texte.
- **Réponses sœurs** - les réponses antérieures au même parent dans le même fil.

Utile pour : tout agent qui répond à un commentaire dans son contexte (agents d'accueil, résumeurs de fil, modérateurs lisant les réponses dans les conversations).

Coût : petit à moyen. Limité par le nombre de réponses sœurs existant dans un fil donné.

#### Inclure le facteur de confiance du commentateur, l'âge du compte, l'historique de bannissement et les commentaires récents

Ajoute le bloc **AUTHOR_HISTORY** :

- **Âge du compte en jours** depuis l'inscription.
- **Facteur de confiance (0-100)** - le score FastComments qui résume à quel point l'utilisateur est digne de confiance sur ce site. Voir la page [Détection de spam](/guide-moderation.html#spam-detection) du guide de modération.
- **Nombre de bannissements antérieurs.**
- **Nombre total de commentaires sur ce site.**
- **Nombre de contenus dupliqués** - si l'utilisateur a publié récemment du texte identique (signal anti-spam).
- **Signal cross-compte même-IP** - nombre de commentaires provenant de la même IP sous d'autres comptes (signal de comptes alternatifs). Le hachage de l'IP lui-même n'est jamais envoyé au LLM.
- **Commentaires récents** - jusqu'à 5 des commentaires les plus récents de l'utilisateur, chacun tronqué à 300 caractères, encadrés comme texte non fiable.

Utile pour : tout agent de modération. Sans cela, le modèle bannit les comptes récents et les utilisateurs de longue date de bonne foi présentant la même posture.

Coût : moyen. Les commentaires récents ajoutent le plus de tokens.

#### Inclure le titre de la page, le sous-titre, la description et les meta tags

Ajoute le bloc **PAGE_CONTEXT** - titre, sous-titre, description et toutes les meta tags que FastComments a capturées pour la page.

Utile pour : agents d'accueil et résumeurs de fil, où savoir de quoi parle la page améliore substantiellement la qualité du résultat.

Coût : faible.

### Règles communautaires

Le quatrième champ, **Règles communautaires**, est un bloc de politique en texte libre inclus dans le message de contexte du rôle utilisateur à chaque exécution, encadré comme texte non fiable de la même manière que les corps de commentaires et autres contenus fournis par les utilisateurs. L'agent le lit comme du texte de politique mais la plateforme ne le traite pas comme une instruction système. Voir [Règles communautaires](#community-guidelines) pour savoir quoi y mettre.

### Ajouter du contexte de manière sélective

Ces cases s'appliquent par agent, pas globalement. Un schéma commun :

- Agent d'accueil : contexte de la page **activé**, contexte du fil **désactivé**, historique utilisateur **désactivé**.
- Modérateur : contexte du fil **désactivé**, historique utilisateur **activé**, contexte de la page **désactivé**.
- Résumeur de fil : contexte du fil **activé**, contexte de la page **activé**, historique utilisateur **désactivé**.

Visez le contexte minimum dont un agent a besoin pour être correct sur les appels qu'il effectue réellement - le contexte supplémentaire coûte des tokens à chaque exécution, même lorsque l'agent ne l'utilise pas.

---