[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazivati značke korisnika samo uz njihove komentare unutar niti komentara.

Međutim, možemo prikazati značke korisnika pored njihovog imena iznad obrasca za komentare omogućivanjem ove značajke na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Ovo će prikazati značke korisnika uz njihovo ime u gornjem području trake, čineći njihova postignuća i status istaknutijima dok pišu komentar.

Napomena da ova značajka mora biti omogućena u sučelju za prilagodbu widgeta da bi radila. Opcionalno možete postaviti zastavicu **showBadgesInTopBar** na false u vašoj konfiguraciji koda kako biste je selektivno onemogućili čak i kada je uključena na razini poslužitelja:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]