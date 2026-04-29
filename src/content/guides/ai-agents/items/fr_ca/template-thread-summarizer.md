**Template ID:** `thread_summarizer`

Le Thread Summarizer publie un résumé neutre en un seul paragraphe à la fin d'un long fil. Il utilise un délai de 30 minutes afin que le fil puisse se stabiliser avant que l'agent n'y jette un coup d'œil.

### Built-in initial prompt

[inline-code-attrs-start title = 'Invite initiale du modèle Thread Summarizer'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

L'instruction "do not editorialize" est essentielle. Sans elle, le modèle a tendance à employer le cadre "à mon avis" qui s'affiche mal sous le nom de compte.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Délai de déclenchement** : 30 minutes (1800 secondes). Voir [Deferred Triggers](#trigger-deferred-delay).

Le délai de 30 minutes signifie que l'agent s'exécute une seule fois, une demi-heure après l'arrivée d'un commentaire, en fonction de l'apparence du fil à ce moment-là. Il ne s'agit pas de "résumer à chaque réponse" — la file d'attente des déclencheurs différés consolide plusieurs événements de nouveau commentaire sur le même fil, mais ne les déduplique pas entre des fenêtres distinctes. Vous souhaiterez probablement **ajouter une règle personnalisée dans votre invite** comme « ne pas publier un nouveau résumé si l'agent a déjà résumé ce fil au cours des dernières 24 heures » et vous appuyer sur le contexte ainsi que sur les [memory tools](#tools-overview) de l'agent pour l'appliquer.

### Allowed tools

- [`write_comment`](#tools-overview) - publie le résumé lui-même.
- [`pin_comment`](#tools-overview) - épingle le résumé afin que les lecteurs le voient en haut du fil.
- [`unpin_comment`](#tools-overview) - désépingler un résumé antérieur du même agent avant d'épingler le nouveau.

Le résumeur ne peut pas modérer ni interagir avec les utilisateurs.

### Pinning the summary

L'agent publie un nouveau commentaire avec `write_comment`, puis appelle `pin_comment` avec l'ID de commentaire retourné. Lors d'exécutions ultérieures sur le même fil, l'invite lui indique d'appeler `unpin_comment` sur son résumé précédent en premier — la plateforme elle-même n'impose pas de règle d'un seul commentaire épinglé par fil, donc laisser le résumé précédent épinglé aboutira à deux résumés épinglés côte à côte. Cochez "Include parent comment and prior replies in the same thread" dans les [Context Options](#context-options) afin que l'agent puisse voir le résumé épinglé précédent.

### Recommended additions before going live

- **Cochez "Include parent comment and prior replies in the same thread"** dans les [Context Options](#context-options). Un résumeur sans contexte de fil est inutile.
- **Ajustez la règle de taille minimale du fil.** « Moins de 5 commentaires » est la valeur par défaut de l'invite, mais dans les communautés actives, 10–20 est plus approprié. Modifiez l'invite directement.
- **Restreignez aux modèles d'URL spécifiques** si vous ne souhaitez des résumés que sur des pages longues, pas sur des annonces ou des pages produit. Voir [Scope: URL and Locale Filters](#scope-url-locale).
- **Surveillez les coûts.** Le résumé est le modèle le plus consommateur de tokens parce qu'il lit l'ensemble du fil à chaque exécution. Fixez un [budget quotidien](#budgets-overview) serré avant de passer en mode Activé.

### Avoiding repeat summaries

L'agent a accès à [`save_memory`](#tools-overview) et [`search_memory`](#tools-overview) - vous pouvez étendre l'invite pour lui demander d'enregistrer des notes « summarized {thread urlId } » et de les vérifier avant de republier. La mémoire est partagée entre tous les agents de votre tenant.