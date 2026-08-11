[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments prikazal uporabniške značke le na njihovih komentarjih znotraj niza komentarjev.

Vendar lahko uporabniške značke prikažemo poleg njihovega imena nad obrazcem za komentar tako, da omogočimo to funkcijo na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Potrdno polje za prikaz značk v zgornji vrstici na strani za prilagajanje gradnika, ki postavi značke poleg imena nad obrazcem za komentar'; title='Možnost prikaza značk v zgornji vrstici' app-screenshot-end]

To bo prikazalo uporabniške značke poleg njihovega imena v območju zgornje vrstice, kar bo njihove dosežke in status naredilo bolj opazen, ko pišejo komentar.

Upoštevajte, da mora biti ta funkcija omogočena v uporabniškem vmesniku za prilagajanje gradnika, da deluje. Po želji lahko v konfiguraciji kode nastavite zastavico **showBadgesInTopBar** na false, da jo izberete onemogočite, tudi če je na strežniku vklopljena:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Onemogoči prikaz značk v zgornji vrstici'; code-example-end]