[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Broj komentara prikazan na vrhu widgeta za komentare može prikazivati ili sve "komentare najviše razine", što znači one odgovore koji su odgovori izravno na stranicu ili članak, ili može biti brojanje **svih** ugniježđenih komentara.

Po zadanom je ovo `true` - to je brojanje potonjeg - svih komentara. U starijim verzijama widgeta za komentare zadana vrijednost je `false`.

Možemo promijeniti ponašanje, tako da bude brojanje **svih** ugniježđenih komentara postavljanjem zastavice **countAll** na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Ako želimo da brojanje odražava samo komentare najviše razine, postavimo zastavicu na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Trenutno se ovo ne može prilagoditi bez promjena u kodu.