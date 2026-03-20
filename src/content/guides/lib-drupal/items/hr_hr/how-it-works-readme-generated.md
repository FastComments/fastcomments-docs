---
Kada korisnik posjeti entitet s omogućenim FastComments poljem:

1. FastComments JavaScript widget se učitava s CDN-a.
2. Ako je SSO konfiguriran, identitet korisnika iz Drupala se prosljeđuje FastCommentsu.
3. Zamjenska opcija `<noscript>` pruža komentare renderirane na poslužitelju za korisnike bez JavaScripta (samo u načinima Live Comments i Streaming Chat).
---