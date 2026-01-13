[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Podrazumevano, novi komentari uživo pojavljuju se na vrhu liste komentara kako se objavljuju u realnom vremenu.

Kada je ova opcija omogućena, novi komentari uživo će umesto toga biti dodati na dno liste. Ovo utiče na način na koji se komentari pojavljuju kada se objavljuju uživo dok korisnici gledaju nit komentara.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Kada je ova postavka omogućena:
- Novi komentari uživo koje objave drugi korisnici pojaviće se na dnu liste komentara
- Korisnici će videti kako se novi komentari pojavljuju ispod postojećih komentara u realnom vremenu
- Ovo utiče samo na ažuriranja komentara uživo - ne na početno učitavanje stranice
- Ovo može pomoći u održavanju toka čitanja kada korisnici prate diskusiju

Imajte na umu da ova postavka utiče samo na mesto gde se novi komentari uživo postavljaju kada stignu u realnom vremenu. Ne utiče na početni redosled sortiranja kada se stranica učitava.