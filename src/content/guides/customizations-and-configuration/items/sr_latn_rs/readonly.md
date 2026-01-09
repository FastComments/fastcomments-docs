[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentarisanje se može zaključati tako da se novi komentari ili glasovi ne mogu ostavljati postavljanjem zastavice readonly na true.

Komentari takođe neće moći da se uređuju ili brišu.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Postavljanje niti komentara na samo za čitanje'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, za čitav domen, ili stranicu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Postavljanje niti komentara na samo za čitanje' app-screenshot-end]

## Ažuriranje!

Od novembra 2022. godine, niti se mogu zaključavati ili otključavati **u realnom vremenu** od strane administratora i moderatora putem menija sa tri tačke iznad polja za odgovore.

Ovo će sprečiti nove komentare, ali će i dalje dozvoljavati glasanje i omogućavati korisnicima da izbrišu svoje komentare ako to žele, dok `readonly` to ne dozvoljava. 

Ovo odgovara polju `isClosed` u `Page` API-ju.

---