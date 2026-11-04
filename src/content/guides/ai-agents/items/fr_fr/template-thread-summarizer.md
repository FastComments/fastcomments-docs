**ID du modèle :** `thread_summarizer`

Le Thread Summarizer publie un résumé neutre d'un seul paragraphe à la fin d'un fil de discussion long. Il utilise un report de 30 minutes afin que le fil puisse se stabiliser avant que l'agent ne l'examine.

Le prompt intégré demande à l'agent de ne pas éditorialiser - c'est essentiel. Sans cela, le modèle tend vers un cadrage « à mon avis » qui rend mal sous le nom d'affichage de votre compte.

### Déclencheurs

- **Nouveau commentaire publié** (`COMMENT_ADD`).
- **Délai du déclencheur** : 30 minutes (1800 secondes). Voir [Déclencheurs différés](#trigger-deferred-delay).

Le délai de 30 minutes signifie que l'agent s'exécute une fois, une demi-heure après l'arrivée d'un commentaire, sur l'état du fil à ce moment-là. Ce n'est pas « résumer à chaque réponse » - la file des déclencheurs différés regroupe plusieurs événements de nouveaux commentaires sur le même fil, mais ne les déduplique pas entre des fenêtres distinctes. Vous voudrez probablement **ajouter une règle personnalisée dans votre prompt** du type « ne pas publier un nouveau résumé si l'agent a déjà résumé ce fil au cours des dernières 24 heures » et vous appuyer sur le contexte ainsi que sur les [outils de mémoire](#tools-overview) de l'agent pour l'appliquer.

### Outils autorisés

- [`write_comment`](#tools-overview) - publie le résumé lui-même.
- [`pin_comment`](#tools-overview) - épingle le résumé afin que les lecteurs le voient en haut du fil.
- [`unpin_comment`](#tools-overview) - désépingle un résumé antérieur du même agent avant d'épingler le nouveau.

Le résumeur ne peut ni modérer ni interagir avec les utilisateurs.

### Épinglage du résumé

L'agent publie un nouveau commentaire avec `write_comment`, puis appelle `pin_comment` avec l'ID du commentaire renvoyé. Lors des exécutions suivantes sur le même fil, le prompt lui demande d'appeler d'abord `unpin_comment` sur son résumé antérieur - la plateforme elle-même n'applique **pas** de règle de commentaire épinglé unique par fil, donc laisser le résumé précédent épinglé donnera deux résumés épinglés côte à côte. Cochez « Inclure le commentaire parent et les réponses antérieures du même fil » dans [Options de contexte](#context-options) pour que l'agent puisse voir le résumé épinglé antérieur.

### Ajouts recommandés avant la mise en production

- **Cochez « Inclure le commentaire parent et les réponses antérieures du même fil »** dans [Options de contexte](#context-options). Un résumeur sans contexte de fil est inutile.
- **Ajustez la règle de taille minimale du fil.** « Moins de 5 commentaires » est la valeur par défaut du prompt, mais dans les communautés actives, 10-20 est plus approprié. Modifiez le prompt directement.
- **Restreignez à des modèles d'URL spécifiques** si vous ne voulez des résumés que sur les pages de type article long, pas sur les annonces ou les pages produits. Voir [Portée : filtres d'URL et de locale](#scope-url-locale).
- **Surveillez les coûts.** La synthèse est le modèle le plus gourmand en jetons car il lit tout le fil à chaque exécution. Définissez un [budget quotidien](#budgets-overview) strict avant de passer à Activé.

### Éviter les résumés répétés

L'agent a accès à [`save_memory`](#tools-overview) et [`search_memory`](#tools-overview) - vous pouvez étendre le prompt pour lui demander d'enregistrer des notes « summarized {thread urlId} » et de les vérifier avant de republier. La mémoire est partagée entre tous les agents de votre tenant.