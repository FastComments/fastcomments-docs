---
[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments omogućava korisnicima da blokiraju druge korisnike. Blokiranje korisnika će uzrokovati da se njihovi komentari maskiraju, sprečava obaveštenja između korisnika i slično.

Možda će biti poželjno onemogućiti ovu funkcionalnost. To se može uraditi na sledeći način:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Onemogući blokiranje'; code-example-end]

Ovo se takođe može uraditi bez koda, što takođe omogućava pravilnu validaciju na serveru, putem UI‑a za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opcija za onemogućavanje blokiranja u UI‑u prilagođavanja widgeta, koja sprečava korisnike da blokiraju jedni druge'; title='Onemogući blokiranje' app-screenshot-end]

---