[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Broj komentara koji se prikazuje na vrhu komentarskog vidžeta može da prikazuje ili sve "top-level" komentare, što znači odgovore koji su direktno odgovori na stranicu ili članak, ili može biti broj **svih** ugnježdenih komentara.

Po defaultu, ovo je `true` - to je broj potonjeg - svi komentari. U starijim verzijama komentarskog vidžeta podrazumevana vrednost je `false`.

Možemo promeniti ponašanje, tako da bude broj **svih** ugnježdenih komentara postavljanjem zastavice **countAll** na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Ako želimo da broj odražava samo komentare najvišeg nivoa, postavimo zastavicu na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Ovo trenutno ne može da se prilagodi bez izmena koda.