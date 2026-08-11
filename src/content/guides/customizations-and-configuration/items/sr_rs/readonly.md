[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentarisanje se može zaključati tako da se ne mogu ostaviti novi komentari ili glasovi postavljanjem zastavice readonly na true.

Komentari takođe neće moći da se uređuju ili brišu.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, za ceo domen ili stranicu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Postavka za sprečavanje novih odgovora na stranici za prilagođavanje widgeta, koja zaključava nit za domen ili stranicu'; title='Postavljanje niti komentara u režim samo za čitanje' app-screenshot-end]

## Ažuriranje!

Od novembra 2022, niti se mogu zaključati ili otključati **uživo** od strane administratora i moderatora putem menija sa tri tačke iznad oblasti za odgovor.

Ovo će sprečiti nove komentare, dok će i dalje omogućiti glasanje i omogućiti korisnicima da izbrišu svoje komentare po želji, dok `readonly` ne dozvoljava ove stvari. 

Ovo odgovara polju `isClosed` u `Page` API-ju.