**ID du modèle :** `thread_summarizer`

Le Thread Summarizer publie un résumé neutre en un seul paragraphe à la fin d'un long fil. Il utilise un délai différé de 30 minutes pour laisser le fil se stabiliser avant que l'agent l'examine.

L'invite intégrée demande à l'agent de ne pas faire d'éditorialisation — c'est essentiel. Sans cela, le modèle tend vers des formules du type « à mon avis » qui s'affichent mal sous le nom de votre compte.

### Déclencheurs

- **Nouveau commentaire publié** (`COMMENT_ADD`).
- **Délai de déclenchement** : 30 minutes (1800 secondes). Voir [Déclencheurs différés](#trigger-deferred-delay).

Le délai de 30 minutes signifie que l'agent s'exécute une seule fois, une demi-heure après la publication d'un commentaire, en fonction de l'état du fil à ce moment-là. Il ne s'agit pas de « résumer à chaque réponse » — la file des déclencheurs différés fusionne plusieurs événements de nouveau commentaire sur le même fil, mais ne les déduplique pas entre des fenêtres distinctes. Vous voudrez probablement **ajouter une règle personnalisée dans votre invite** comme « ne publiez pas un nouveau résumé si l'agent a déjà résumé ce fil au cours des dernières 24 heures » et vous appuyer sur le contexte ainsi que sur les [outils de mémoire](#tools-overview) de l'agent pour le faire respecter.

### Outils autorisés

- [`write_comment`](#tools-overview) - publie le résumé lui-même.
- [`pin_comment`](#tools-overview) - épingle le résumé pour que les lecteurs le voient en haut du fil.
- [`unpin_comment`](#tools-overview) - désépinglera un résumé précédent du même agent avant d'épingler le nouveau.

Le résumeur ne peut pas modérer ni interagir avec les utilisateurs.

### Épingler le résumé

L'agent publie un nouveau commentaire avec `write_comment`, puis appelle `pin_comment` avec l'ID du commentaire retourné. Lors des exécutions suivantes sur le même fil, l'invite lui demande d'appeler d'abord `unpin_comment` sur son résumé précédent — la plateforme elle-même n'impose **pas** de règle d'un seul commentaire épinglé par fil, donc laisser l'ancien résumé épinglé aboutira à deux résumés épinglés côte à côte. Cochez « Inclure le commentaire parent et les réponses antérieures dans le même fil » dans [Options de contexte](#context-options) pour que l'agent puisse voir le résumé épinglé précédent.

### Ajouts recommandés avant la mise en production

- **Cochez « Inclure le commentaire parent et les réponses antérieures dans le même fil »** dans [Options de contexte](#context-options). Un résumeur sans contexte de fil est inutile.
- **Ajustez la règle de taille minimale du fil.** « Moins de 5 commentaires » est la valeur par défaut de l'invite, mais dans des communautés actives 10–20 est souvent plus approprié. Modifiez l'invite directement.
- **Restreignez aux motifs d'URL spécifiques** si vous ne voulez des résumés que sur des pages longues, et non sur des annonces ou des pages produit. Voir [Périmètre : filtres URL et de langue](#scope-url-locale).
- **Surveillez les coûts.** La summarisation est le modèle le plus consommateur de tokens car il lit l'ensemble du fil à chaque exécution. Définissez un [budget quotidien](#budgets-overview) strict avant de passer à Activé.

### Éviter les résumés répétés

L'agent a accès à [`save_memory`](#tools-overview) et [`search_memory`](#tools-overview) — vous pouvez étendre l'invite pour lui demander d'enregistrer des notes du type "résumé {thread urlId}" et de les vérifier avant de republier. La mémoire est partagée entre tous les agents de votre locataire.

---