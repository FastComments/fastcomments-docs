Da bi dodatak radio, token se sprema u vašu WordPress bazu podataka i na vaš FastComments nalog. Kada dodatak šalje zahtjev našim serverima, on dostavlja taj token.

Možete pogledati sve integracije ovlaštene za vaš FastComments nalog [ovdje](https://fastcomments.com/auth/my-account/manage-data/integrations).

Sva komunikacija se obavlja putem HTTPS-a.

Sva komunikacija je *izlazna* sa vašeg WordPress servera *prema* FastComments.com, uključujući i sinhronizaciju *natrag* ka vašoj WordPress instalaciji, jer se ona implementira putem [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) iz [cron](https://developer.wordpress.org/plugins/cron/) podešavanja u vašoj WordPress instalaciji.

---