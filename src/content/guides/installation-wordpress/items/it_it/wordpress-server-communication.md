Perché il plugin funzioni, un token viene salvato nel tuo database di WordPress e anche nel tuo account FastComments. Quando il plugin effettua richieste ai nostri server, fornisce
questo token.

Puoi visualizzare tutte le integrazioni autorizzate nel tuo account FastComments [qui](https://fastcomments.com/auth/my-account/manage-data/integrations).

Tutta la comunicazione avviene tramite HTTPS.

Tutta la comunicazione è *in uscita* dal tuo server WordPress *verso* FastComments.com, inclusa la sincronizzazione *di ritorno* alla tua installazione WordPress poiché è implementata
via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) da una [cron](https://developer.wordpress.org/plugins/cron/) impostazione nella tua installazione WordPress.