[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentiranje se može zaključati tako da se postavljanjem zastavice readonly na true onemogućuje ostavljanje novih komentara ili glasova.

Komentari također neće moći biti uređivani ili izbrisani.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Postavljanje niti komentara na samo čitanje'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta, za cijelu domenu ili stranicu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Postavka za sprječavanje novih odgovora na stranici za prilagodbu widgeta, koja zaključava nit za domenu ili stranicu'; title='Postavljanje niti komentara na samo čitanje' app-screenshot-end]

## Ažuriranje!

Od studenog 2022., niti se mogu zaključati ili otključati **uživo** od strane administratora i moderatora putem izbornika s tri točke iznad područja za odgovor.

Ovo će spriječiti nove komentare, dok će i dalje omogućiti glasanje i omogućiti korisnicima da izbrišu svoje komentare po želji, dok `readonly` ne dopušta te stvari. 

Ovo odgovara polju `isClosed` u `Page` API-ju.

---