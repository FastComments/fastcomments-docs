[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments prikazal značke uporabnikov le pri njihovih komentarjih v niti komentarjev.

Vendar pa lahko značke uporabnikov prikažemo poleg njihovega imena nad obrazcem za komentar z omogočitvijo te funkcije na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

To bo prikazalo značke uporabnika poleg njegovega imena v zgornjem pasu, kar bo njihove dosežke in status bolj izpostavilo, ko sestavljajo komentar.

Upoštevajte, da mora biti ta funkcija omogočena v uporabniškem vmesniku za prilagajanje gradnika, da deluje. Po želji lahko v konfiguraciji kode zastavico **showBadgesInTopBar** nastavite na false, da jo selektivno onemogočite, tudi ko je omogočena na ravni strežnika:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---