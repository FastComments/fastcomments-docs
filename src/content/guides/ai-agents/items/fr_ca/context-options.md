---
La section **Contexte** du formulaire d'édition contrôle la quantité d'informations que l'agent reçoit à chaque exécution. Plus de contexte permet de meilleures décisions mais augmente le coût en jetons par exécution, vous ne voulez donc que ce dont l'agent a réellement besoin.

### Ce qui est toujours inclus

Même si toutes les cases sont décochées, le message de contexte envoyé à l'agent comprend :

- Le **type d'événement déclencheur** (p. ex. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL de la page et l'ID d'URL** (lorsque connus).
- Le **commentaire** qui a déclenché l'exécution, s'il y en a un - ID, ID de l'utilisateur auteur, nom d'affichage de l'auteur, texte du commentaire, décomptes de votes, nombre de signalements, indicateurs spam/approuvé/relue, ID du parent. L'adresse courriel de l'auteur n'est **jamais** envoyée au fournisseur de LLM (minimisation des PII).
- Le **texte du commentaire précédent** pour les déclencheurs `COMMENT_EDIT` (afin que l'agent puisse comparer avant/après).
- La **direction du vote** pour les déclencheurs `COMMENT_VOTE_THRESHOLD`.
- L'**ID de l'utilisateur déclencheur** et l'**ID du badge** (pour les déclencheurs de badge de modérateur).
- Le **catalogue de badges** de votre locataire (nom, label d'affichage, description) lorsque l'agent est autorisé à attribuer des badges, afin que l'agent puisse en choisir un approprié sans que vous ayez à les détailler dans l'invite.

Tous les textes non fiables - les corps de commentaires, les noms d'auteurs, les titres de pages, le document de directives lui-même - sont **encadrés** dans le message de contexte avec des marqueurs comme `<<<COMMENT_TEXT>>> ... <<<END>>>`. L'invite système de la plateforme ordonne au modèle de ne jamais exécuter les instructions situées à l'intérieur de ces encadrements. Il s'agit de la défense de la plateforme contre l'injection d'invite ; vous n'avez pas besoin de la répéter dans votre invite.

### Les trois cases à cocher

#### Inclure le commentaire parent et les réponses précédentes dans le même fil

Ajoute :
- Le **commentaire parent** - ID, auteur, texte.
- **Réponses sœurs** - les réponses antérieures au même parent dans le même fil.

Utile pour : tout agent qui répond à un commentaire en contexte (accueilleurs automatiques, résumeurs de fil, modérateurs lisant les réponses dans les conversations).

Coût : petit à moyen. Limité par le nombre de réponses sœurs existant dans un fil donné.

#### Inclure le facteur de confiance du commentateur, l'ancienneté du compte, l'historique des bannissements et les commentaires récents

Ajoute le bloc **AUTHOR_HISTORY** :

- **Âge du compte en jours** depuis l'inscription.
- **Facteur de confiance (0-100)** - le score FastComments qui résume la fiabilité de l'utilisateur sur ce site. Voir la page [Détection du spam](/guide-moderation.html#spam-detection) du guide de modération.
- **Nombre de bannissements antérieurs.**
- **Nombre total de commentaires sur ce site.**
- **Nombre de contenus dupliqués** - si l'utilisateur a récemment publié un texte identique (signal anti-spam).
- **Signal de comptes croisés par la même IP** - nombre de commentaires provenant de la même IP sous d'autres comptes (signal de comptes alternatifs). Le hachage de l'IP lui-même n'est jamais envoyé au LLM.
- **Commentaires récents** - jusqu'à 5 des commentaires les plus récents de l'utilisateur, chacun tronqué à 300 caractères, encadrés comme texte non fiable.

Utile pour : tout agent de modération. Sans cela, le modèle bannit les nouveaux comptes et les utilisateurs de longue date de bonne foi présentant le même comportement.

Coût : moyen. Les commentaires récents ajoutent le plus de jetons.

#### Inclure le titre de la page, le sous-titre, la description et les balises méta

Ajoute le bloc **PAGE_CONTEXT** - titre, sous-titre, description et toutes les balises méta que FastComments a capturées pour la page.

Utile pour : les accueilleurs automatiques et les résumeurs de fil, où connaître le sujet de la page améliore considérablement la qualité de la sortie.

Coût : faible.

### Directives communautaires

Le quatrième champ, **Directives communautaires**, est un bloc de politique en texte libre inclus dans le message de contexte du rôle utilisateur à chaque exécution, encadré comme texte non fiable de la même manière que les corps de commentaires et autres contenus fournis par les utilisateurs. L'agent le lit comme du texte de politique mais la plateforme ne le considère pas comme une instruction système. Voir [Directives communautaires](#community-guidelines) pour ce qu'il faut y mettre.

### Ajouter du contexte de manière sélective

Ces cases s'appliquent par agent, et non globalement. Un schéma courant :

- Agent d'accueil : contexte de page **activé**, contexte de fil **désactivé**, historique utilisateur **désactivé**.
- Modérateur : contexte de fil **désactivé**, historique utilisateur **activé**, contexte de page **désactivé**.
- Résumeur de fil : contexte de fil **activé**, contexte de page **activé**, historique utilisateur **désactivé**.

Visez le contexte minimum dont un agent a besoin pour être correct dans les appels qu'il effectue réellement - un contexte supplémentaire coûte des jetons à chaque exécution, même lorsque l'agent ne l'utilise pas.

---