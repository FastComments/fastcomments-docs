[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments korisnicima omogućuje blokiranje drugih korisnika. Blokiranje korisnika uzrokovat će da njihovi komentari
budu zamagljeni, spriječiti će obavijesti između korisnika i slično.

Možda će biti poželjno onemogućiti ovu funkcionalnost. To se može učiniti na sljedeći način:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Ovo se također može učiniti bez koda, što također omogućuje ispravnu validaciju na strani poslužitelja, putem sučelja za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---