[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Po podrazumevanju, FastComments prikazuje značke korisnika samo na njihovim komentarima unutar niti komentara.

Međutim, možemo prikazati značke korisnika pored njihovog imena iznad obrasca za komentare omogućavanjem ove funkcije na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Ovo će prikazati značke korisnika pored njihovog imena u gornjem delu trake, čineći njihova postignuća i status istaknutijim dok pišu komentar.

Imajte na umu da ova funkcija mora biti omogućena u UI-ju za prilagođavanje widgeta da bi radila. Opcionalno možete postaviti zastavicu **showBadgesInTopBar** na false u konfiguraciji svog koda da biste je selektivno onemogućili čak i kada je uključena na serverskom nivou:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---