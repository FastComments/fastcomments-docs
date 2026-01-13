[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Brojač komentara koji se prikazuje na vrhu komentarskog vidžeta može da prikaže ili sve "komentare najvišeg nivoa", što znači one odgovore koji su odgovori direktno na stranicu ili članak, ili može biti broj **svih** ugnježdenih komentara.

Podrazumevano je `true` - radi se o brojanju potonjeg - svih komentara. U starijim verzijama komentarskog vidžeta podrazumevana vrednost je `false`.

Možemo promeniti ponašanje, tako da bude broj **svih** ugnježdenih komentara postavljanjem zastavice **countAll** na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Ako želimo da broj odražava samo komentare najvišeg nivoa, postavimo zastavicu na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Trenutno se ovo ne može prilagoditi bez izmena koda.

---