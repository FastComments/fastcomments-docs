---
[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, novi komentari uživo pojavljuju se na vrhu popisa komentara kako se objavljuju u stvarnom vremenu.

Kada je ova opcija omogućena, novi komentari uživo bit će umjesto toga dodavani na dno liste. Ovo utječe na način na koji se komentari prikazuju kada se objavljuju uživo dok korisnici pregledavaju nit komentara.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Kada je ova postavka omogućena:
- Novi komentari uživo koje objave drugi korisnici pojavit će se na dnu liste komentara
- Korisnici će u stvarnom vremenu vidjeti nove komentare koji se pojavljuju ispod postojećih komentara
- Ovo utječe samo na ažuriranja komentara uživo - ne i na inicijalno učitavanje stranice
- To može pomoći u održavanju toka čitanja kada korisnici prate raspravu

Imajte na umu da ova postavka utječe samo na mjesto na koje se novi komentari uživo smještaju kada stignu u stvarnom vremenu. Ne utječe na početni redoslijed sortiranja pri učitavanju stranice.
---