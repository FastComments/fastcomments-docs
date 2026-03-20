Kada korisnik poseti entitet sa omogućеним FastComments poljem:

1. FastComments JavaScript widget se učitava sa CDN-a.
2. Ako je SSO konfigurisan, Drupal identitet korisnika se prosleđuje FastComments-u.
3. Rezervna opcija `<noscript>` obezbeđuje komentare renderovane na serveru za korisnike bez JavaScripta (samo u režimima Live Comments i Streaming Chat).