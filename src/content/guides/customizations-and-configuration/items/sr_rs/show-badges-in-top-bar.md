[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Po podrazumevanoj postavci, FastComments prikazuje bedževe korisnika samo pored njihovih komentara u okviru niti komentara.

Međutim, možemo prikazati bedževe korisnika pored njihovog imena iznad forme za komentar omogućavanjem ove funkcije na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Ovo će prikazati bedževe korisnika pored njihovog imena u gornjoj traci, čineći njihova postignuća i status istaknutijim dok pišu komentar.

Imajte na umu da ova funkcija mora biti omogućena u interfejsu za prilagođavanje widgeta da bi radila. Po želji možete postaviti zastavicu **showBadgesInTopBar** na false u konfiguraciji koda da biste je selektivno onemogućili čak i kada je uključena na nivou servera:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]