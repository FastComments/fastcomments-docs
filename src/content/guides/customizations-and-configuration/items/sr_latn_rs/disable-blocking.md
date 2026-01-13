[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments omogućava korisnicima da blokiraju druge korisnike. Blokiranje korisnika će prouzrokovati da njegovi komentari budu maskirani,
sprečava slanje obaveštenja između korisnika i slično.

Možda je poželjno onemogućiti ovu funkcionalnost. To se može uraditi na sledeći način:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Ovo se takođe može uraditi bez koda, što takođe omogućava ispravnu validaciju na serverskoj strani, putem Interfejsa za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]