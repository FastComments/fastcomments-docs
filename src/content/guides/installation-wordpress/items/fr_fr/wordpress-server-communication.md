Pour que le plugin fonctionne, un jeton est enregistré dans votre base de données WordPress et également dans votre compte FastComments. Lorsque le plugin envoie une requête à nos serveurs, il fournit
ce jeton.

Vous pouvez consulter toutes les intégrations autorisées pour votre compte FastComments [ici](https://fastcomments.com/auth/my-account/manage-data/integrations).

Toutes les communications se font via HTTPS.

Toute communication est *sortante* depuis votre serveur WordPress *vers* FastComments.com, y compris la synchronisation *de retour* vers votre installation WordPress, car elle est mise en œuvre
via [interrogation périodique](https://en.wikipedia.org/wiki/Polling_(computer_science)) à partir d'une configuration [cron](https://developer.wordpress.org/plugins/cron/) dans votre installation WordPress.