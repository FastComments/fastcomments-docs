Om de plugin te laten werken wordt een token opgeslagen in uw WordPress-database en ook in uw FastComments-account. Wanneer de plugin verzoeken naar onze servers stuurt, levert het
dit token.

U kunt alle integraties die geautoriseerd zijn voor uw FastComments-account [hier](https://fastcomments.com/auth/my-account/manage-data/integrations) bekijken.

Alle communicatie vindt plaats via HTTPS.

Alle communicatie is *uitgaand* van uw WordPress-server *naar* FastComments.com, inclusief de synchronisatie *terug* naar uw WordPress-installatie aangezien deze wordt geïmplementeerd
via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) vanuit een [cron](https://developer.wordpress.org/plugins/cron/) configuratie in uw WordPress-installatie.