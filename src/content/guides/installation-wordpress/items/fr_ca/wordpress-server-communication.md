Pour que le plugin fonctionne, un jeton est enregistré dans votre base de données WordPress et aussi dans votre compte FastComments. Lorsque le plugin envoie une requête à nos serveurs, il fournit
ce jeton.

Vous pouvez voir toutes les intégrations autorisées pour votre compte FastComments [ici](https://fastcomments.com/auth/my-account/manage-data/integrations).

Toute la communication se fait via HTTPS.

Toute la communication est *sortante* depuis votre serveur WordPress *vers* FastComments.com, y compris la synchronisation *de retour* vers votre installation WordPress puisqu'elle est implémentée
via [sondage](https://en.wikipedia.org/wiki/Polling_(computer_science)) depuis une configuration [cron](https://developer.wordpress.org/plugins/cron/) dans votre installation WordPress.