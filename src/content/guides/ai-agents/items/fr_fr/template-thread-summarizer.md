**Template ID:** `thread_summarizer`

Le Thread Summarizer publie un résumé neutre en un seul paragraphe à la fin d'un long fil. Il utilise un délai de 30 minutes pour laisser le fil se stabiliser avant que l'agent ne l'examine.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle Thread Summarizer'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

L'instruction "do not editorialize" est essentielle. Sans elle, le modèle tend à adopter un cadrage de type "in my view" qui se lit mal sous le nom d'affichage de votre compte.

### Déclencheurs

- **Nouveau commentaire publié** (`COMMENT_ADD`).
- **Délai de déclenchement** : 30 minutes (1800 secondes). Voir [Déclencheurs différés](#trigger-deferred-delay).

Le délai de 30 minutes signifie que l'agent s'exécute une seule fois, une demi-heure après la publication d'un commentaire, par rapport à l'état du fil à ce moment-là. Il ne s'agit pas de "résumer à chaque réponse" - la file d'attente des déclencheurs différés rassemble plusieurs événements de nouveau commentaire sur le même fil, mais ne les dé-duplique pas sur des fenêtres séparées. Vous voudrez probablement **ajouter une règle personnalisée dans votre invite** comme "do not post a new summary if the agent has already summarized this thread in the last 24 hours" et vous appuyer sur le contexte ainsi que les [outils de mémoire](#tools-overview) de l'agent pour l'appliquer.

### Outils autorisés

- [`write_comment`](#tools-overview) - publie le résumé lui-même.
- [`pin_comment`](#tools-overview) - épingle le résumé pour que les lecteurs le voient en haut du fil.
- [`unpin_comment`](#tools-overview) - désépinglera un résumé antérieur du même agent avant d'épingler le nouveau.

Le système de résumé ne peut pas modérer ni interagir avec les utilisateurs.

### Épingler le résumé

L'agent publie un nouveau commentaire avec `write_comment`, puis appelle `pin_comment` avec l'ID de commentaire retourné. Lors d'exécutions ultérieures sur le même fil, l'invite lui demande d'appeler `unpin_comment` sur son résumé précédent d'abord - la plateforme elle-même n'impose pas de règle d'un seul commentaire épinglé par fil, donc laisser le résumé précédent épinglé aboutira à deux résumés épinglés côte à côte. Cochez "Include parent comment and prior replies in the same thread" dans [Options de contexte](#context-options) afin que l'agent puisse voir le résumé épinglé précédent.

### Ajouts recommandés avant la mise en production

- **Cochez "Include parent comment and prior replies in the same thread"** dans [Options de contexte](#context-options). Un résumeur sans contexte de fil est inutile.
- **Ajustez la règle de taille minimale du fil.** "Fewer than 5 comments" est la valeur par défaut de l'invite, mais dans les communautés actives, 10-20 est plus approprié. Modifiez l'invite directement.
- **Restreignez aux motifs d'URL spécifiques** si vous souhaitez des résumés uniquement sur les pages longues, pas sur les annonces ou les pages produit. Voir [Portée : filtres d'URL et de localisation](#scope-url-locale).
- **Surveillez les coûts.** La synthèse est le modèle le plus gourmand en tokens car il lit l'ensemble du fil à chaque exécution. Fixez un [budget quotidien](#budgets-overview) strict avant de passer en Enabled.

### Éviter les résumés répétés

L'agent a accès à [`save_memory`](#tools-overview) et [`search_memory`](#tools-overview) - vous pouvez étendre l'invite pour lui demander d'enregistrer des notes "summarized {thread urlId}" et de les vérifier avant de republier. La mémoire est partagée entre tous les agents de votre client.

---