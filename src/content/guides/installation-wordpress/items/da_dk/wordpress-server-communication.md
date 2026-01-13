---
For at pluginet fungerer, gemmes et token i din WordPress-database og også på din FastComments-konto. Når pluginet laver forespørgsler til vores servere, medbringer det
dette token.

Du kan se alle integrationer, der er autoriseret til din FastComments-konto [her](https://fastcomments.com/auth/my-account/manage-data/integrations).

Al kommunikation foregår via HTTPS.

Al kommunikation er *udgående* fra din WordPress-server *til* FastComments.com, inklusive synkroniseringen *tilbage* til din WordPress-installation, da den er implementeret
via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) fra en [cron](https://developer.wordpress.org/plugins/cron/) opsætning i din WordPress-installation.
---