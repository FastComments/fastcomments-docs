[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments prikazuje značke korisnika samo na njihovim komentarima unutar niti komentara.

Međutim, možemo prikazati značke korisnika pored njihovog imena iznad obrasca za komentare tako što ćemo omogućiti ovu opciju na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Ovo će prikazati značke korisnika pored njihovog imena u oblasti gornje trake, čineći njihova dostignuća i status istaknutijim dok sastavljaju komentar.

Napomena: ova opcija mora biti omogućena u korisničkom interfejsu za prilagođavanje widgeta da bi radila. Opcionalno možete postaviti zastavicu **showBadgesInTopBar** na false u svojoj konfiguraciji koda da je selektivno onemogućite čak i kada je uključena na nivou servera:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]