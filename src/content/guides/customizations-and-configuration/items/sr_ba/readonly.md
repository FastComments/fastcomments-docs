[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentarisanje se može zaključati tako da se ne mogu ostaviti novi komentari ili glasovi postavljanjem zastavice `readonly` na true.

Komentari takođe neće moći biti uređivani ili obrisani.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta, za čitav domen ili stranicu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Ažuriranje!

Od novembra 2022., teme se mogu zaključavati ili otključavati **uživo** od strane administratora i moderatora putem menija s tri tačkice iznad oblasti za odgovor.

To će spriječiti nove komentare, ali će i dalje dozvoliti glasanje i omogućiti korisnicima da po potrebi obrišu svoje komentare, dok `readonly` to ne dozvoljava. 

Ovo odgovara polju `isClosed` u `Page` API.