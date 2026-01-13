[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentiranje se može zaključati tako da se postavi zastavica readonly na true, čime će biti onemogućeno ostavljanje novih komentara ili davanje glasova.

Komentari također neće moći biti uređivani ili izbrisani.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Postavljanje niti komentara u način samo za čitanje'; code-example-end]

To se može prilagoditi bez kodiranja, na stranici za prilagodbu widgeta, za cijeli domen ili pojedinu stranicu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Postavljanje niti komentara u način samo za čitanje' app-screenshot-end]

## Ažuriranje!

Od studenog 2022., niti mogu biti zaključane ili otključane **uživo** od strane administratora i moderatora putem izbornika s tri točkice iznad područja za odgovor.

To će spriječiti nove komentare, ali će i dalje omogućavati glasanje te korisnicima omogućiti brisanje svojih komentara ako to žele, dok `readonly` to ne dopušta. 

Ovo odgovara polju `isClosed` u `Page` API-ju.