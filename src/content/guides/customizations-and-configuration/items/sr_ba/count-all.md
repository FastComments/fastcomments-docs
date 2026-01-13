[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Broj komentara prikazan na vrhu widgeta za komentare može ili prikazivati sve "top-level" komentare, što znači one odgovore koji
su odgovori direktno na stranicu ili članak, ili može biti broj **svih** ugniježdenih komentara.

Po zadanoj postavci, ovo je `true` - to je broj potonjeg - svih komentara. U starijim verzijama widgeta za komentare zadana vrijednost je `false`.

Možemo promijeniti ponašanje, tako da predstavlja broj **svih** ugniježdenih komentara postavljanjem zastavice **countAll** na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Ako želimo da broj odražava samo komentare najvišeg nivoa, postavimo zastavicu na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Ovo trenutno ne može biti prilagođeno bez izmjena koda.