[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments omogućuje korisnicima da blokiraju druge korisnike. Blokiranje korisnika će uzrokovati da se njihovi komentari maskiraju, sprječava obavijesti između korisnika i slično.

Možda će biti poželjno onemogućiti ovu funkcionalnost. To se može učiniti na sljedeći način:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Onemogući blokiranje'; code-example-end]

Ovo se također može učiniti bez koda, što također omogućuje pravilnu provjeru na strani poslužitelja, putem UI‑a za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opcija za onemogućavanje blokiranja u UI prilagodbe widgeta, koja sprječava korisnike da blokiraju jedni druge'; title='Onemogući blokiranje' app-screenshot-end]