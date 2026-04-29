**Template ID:** `thread_summarizer`

Le Thread Summarizer publie un résumé neutre, en un seul paragraphe, à la fin d'un long fil. Il utilise un délai de 30 minutes afin que le fil puisse se stabiliser avant que l'agent ne l'examine.

L'invite intégrée demande à l'agent de ne pas éditorialiser — c'est crucial. Sans cela, le modèle a tendance à adopter des tournures du type "à mon avis" qui portent mal à l'écran sous le nom d'affichage de votre compte.

### Triggers

- **Nouveau commentaire publié** (`COMMENT_ADD`).
- **Délai du déclencheur** : 30 minutes (1800 secondes). Voir [Déclencheurs différés](#trigger-deferred-delay).

Le délai de 30 minutes signifie que l'agent s'exécute une seule fois, une demi-heure après l'arrivée d'un commentaire, sur l'état du fil à ce moment-là. Il ne s'agit pas de "résumer à chaque réponse" — la file d'attente des déclencheurs différés fusionne plusieurs événements de nouveau commentaire sur le même fil, mais ne les déduplique pas à travers des fenêtres séparées. Vous voudrez probablement **ajouter une règle personnalisée dans votre invite** comme "ne pas publier un nouveau résumé si l'agent a déjà résumé ce fil au cours des dernières 24 heures" et compter sur le contexte ainsi que sur les [outils de mémoire](#tools-overview) de l'agent pour l'appliquer.

### Allowed tools

- [`write_comment`](#tools-overview) - publie le résumé lui-même.
- [`pin_comment`](#tools-overview) - épingle le résumé pour que les lecteurs le voient en haut du fil.
- [`unpin_comment`](#tools-overview) - désépingler un résumé précédent du même agent avant d'épingler le nouveau.

Le générateur de résumés ne peut pas modérer ni interagir avec les utilisateurs.

### Pinning the summary

L'agent publie un nouveau commentaire avec `write_comment`, puis appelle `pin_comment` avec l'ID du commentaire retourné. Lors des exécutions suivantes sur le même fil, l'invite lui demande d'appeler `unpin_comment` sur son résumé précédent en premier — la plateforme elle-même n'applique **pas** la règle d'un seul commentaire épinglé par fil, donc laisser l'ancien résumé épinglé aboutira à deux résumés épinglés côte à côte. Cochez "Include parent comment and prior replies in the same thread" dans [Options de contexte](#context-options) afin que l'agent puisse voir le résumé épinglé précédent.

### Recommended additions before going live

- **Cochez "Include parent comment and prior replies in the same thread"** dans [Options de contexte](#context-options). Un générateur de résumés sans contexte de fil est inutile.
- **Ajustez la règle de taille minimale du fil.** "Moins de 5 commentaires" est la valeur par défaut de l'invite, mais dans les communautés actives 10–20 est plus approprié. Modifiez l'invite directement.
- **Restreignez aux motifs d'URL spécifiques** si vous ne souhaitez des résumés que sur des pages longues, pas sur des annonces ou des pages produit. Voir [Portée : filtres d'URL et de langue](#scope-url-locale).
- **Surveillez les coûts.** La synthèse est le modèle le plus gourmand en tokens car il lit tout le fil à chaque exécution. Fixez un [budget quotidien](#budgets-overview) strict avant de passer en mode Activé.

### Avoiding repeat summaries

L'agent a accès à [`save_memory`](#tools-overview) et [`search_memory`](#tools-overview) — vous pouvez étendre l'invite pour lui demander d'enregistrer des notes "résumé {thread urlId}" et de les vérifier avant de publier de nouveau. La mémoire est partagée entre tous les agents de votre locataire.