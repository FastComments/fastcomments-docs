Da bi dodatak radio, token se sprema u vašu WordPress bazu podataka i također na vaš FastComments račun. Kad dodatak pošalje zahtjev našim poslužiteljima, on pruža
ovaj token.

Sve integracije autorizirane za vaš FastComments račun možete pregledati [ovdje](https://fastcomments.com/auth/my-account/manage-data/integrations).

Sva komunikacija se obavlja putem HTTPS-a.

Sva komunikacija je *odlazna* s vašeg WordPress servera *prema* FastComments.com, uključujući i sinkronizaciju *natrag* na vašu WordPress instalaciju jer se provodi
putem [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) iz [cron](https://developer.wordpress.org/plugins/cron/) podešenja u vašoj WordPress instalaciji.