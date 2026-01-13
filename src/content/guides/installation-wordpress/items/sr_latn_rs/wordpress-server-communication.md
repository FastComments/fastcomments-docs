Da bi dodatak funkcionisao, token se čuva u vašoj WordPress bazi podataka i takođe na vašem FastComments nalogu. Kada dodatak šalje zahtev našim serverima, on dostavlja
ovaj token.

Sve integracije koje su autorizovane za vaš FastComments nalog možete pogledati [ovde](https://fastcomments.com/auth/my-account/manage-data/integrations).

Sva komunikacija se obavlja putem HTTPS-a.

Sva komunikacija je *izlazna* sa vašeg WordPress servera *ka* FastComments.com, uključujući sinhronizaciju *natrag* na vašu WordPress instalaciju kako je ona implementirana
putem [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) iz [cron](https://developer.wordpress.org/plugins/cron/) podešavanja u vašoj WordPress instalaciji.