[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će prikazivati korisničke značke samo na njihovim komentarima unutar niti komentara.

Međutim, možemo prikazati korisničke značke pored njihovog imena iznad forme za komentar omogućavanjem ove funkcije na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Polje za prikaz znački u gornjoj traci na stranici za prilagođavanje widgeta, postavljanje znački pored imena iznad forme za komentar'; title='Opcija Prikaza Značaka u Gornjoj Traci' app-screenshot-end]

Ovo će prikazati korisničke značke pored njihovog imena u području gornje trake, čineći njihove postignuće i status vidljivijim dok pišu komentar.

Napomena da ova funkcija mora biti omogućena u UI‑u za prilagođavanje widgeta da bi radila. Opcionalno možete postaviti **showBadgesInTopBar** zastavicu na false u vašoj konfiguraciji koda da je selektivno onemogućite čak i kada je uključena na nivou servera:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Onemogući Prikaz Značaka u Gornjoj Traci'; code-example-end]
---