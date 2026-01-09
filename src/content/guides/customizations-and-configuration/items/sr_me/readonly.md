[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentarisanje se može zaključati tako da se ne mogu ostavljati novi komentari ili glasovi postavljanjem zastavice readonly na true.

Komentari такође neće moћи да се uređuju или бришу.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видгета, за читав домен, или страницу:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Ažuriranje!

Od novembra 2022. niti se mogu zaključavati ili otključavati **uživo** od strane administratora i moderatora putem menija sa tri tačke iznad polja za odgovor.

Ovo će sprečiti nove komentare, a istovremeno omogućavati glasanje i dozvoliti korisnicima da obrišu svoje komentare ako to žele, dok `readonly` to ne dozvoljava. 

Ovo odgovara polju `isClosed` u `Page` API-ju.

---