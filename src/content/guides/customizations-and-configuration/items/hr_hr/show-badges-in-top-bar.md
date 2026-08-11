[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazivati značke korisnika samo na njihovim komentarima unutar niti komentara.

Međutim, možemo prikazati značke korisnika pored njihovog imena iznad obrasca za komentar omogućavanjem ove značajke na stranici prilagodbe widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Potvrdni okvir za prikaz znački u gornjoj traci na stranici prilagodbe widgeta, postavljanje znački pored imena iznad obrasca za komentar'; title='Opcija prikaza znački u gornjoj traci' app-screenshot-end]

Ovo će prikazati značke korisnika uz njihovo ime u području gornje trake, čineći njihove postignuće i status vidljivijim dok pišu komentar.

Napomena da ova značajka mora biti omogućena u sučelju za prilagodbu widgeta da bi radila. Opcionalno možete postaviti zastavicu **showBadgesInTopBar** na false u konfiguraciji koda kako biste je selektivno onemogućili čak i kada je uključena na razini poslužitelja:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Onemogući prikaz znački u gornjoj traci'; code-example-end]