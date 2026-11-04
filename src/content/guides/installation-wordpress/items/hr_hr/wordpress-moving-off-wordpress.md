Ako premještate svoju web-lokaciju s WordPressa i želite FastComments na novoj lokaciji, ne treba vam WordPress dodatak. Izvezite svoje komentare iz WordPressa, a zatim prenesite datoteku na [Import page](https://fastcomments.com/auth/my-account/manage-data/import) u FastComments nadzornoj ploči.

Podržavamo dva formata WordPress izvoza.

### WordPress XML (preporučeno)

Ovo je datoteka iz WordPressovog ugrađenog izvoznika, pa nije potreban dodatni dodatak.

1. U svom WordPress administratorskom sučelju idite na `Tools -> Export`.
2. Odaberite `All content` i kliknite `Download Export File`.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import) odaberite `WordPress (.xml)` i prenesite datoteku.

Svaki komentar povezan je s URL-om objave na kojoj je ostavljen, a taj URL je već u datoteci.

Import zadržava ime autora, e-poštu i web-stranicu, datum, sadržaj, razinu odgovora (threading) te je li komentar odobren. Avatari komentatora preneseni su s Gravatara. Glasovi nisu dio ovog formata.

### WordPress CSV

Ovo je datoteka iz [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Instalirajte dodatak u svom WordPress administratorskom sučelju i izvezite komentare kao CSV.
2. Zamijenite svaku vrijednost `comment_post_ID` URL-om objave.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import) odaberite `WordPress (.csv)` i prenesite datoteku.

Svaki komentar povezan je sa stupcem `comment_post_ID`. WordPress u taj stupac upisuje ID objave, a vaša nova lokacija nema WordPress ID-jeve objava, pa ga korak 2 zamjenjuje URL-om.

Import zadržava ime autora, e-poštu i web-stranicu, datum, sadržaj, razinu odgovora (threading) te je li komentar odobren. Avatari komentatora preneseni su s Gravatara. Također zadržava WordPressovu oznaku za spam te wpDiscuz sviđanja i nesviđanja ako ih datoteka sadrži.

### Povezivanje komentara s vašim novim stranicama

Ako vaša nova lokacija koristi iste URL-ove kao vaša WordPress lokacija, komentari se pojavljuju na odgovarajućim stranicama bez dodatnih postavki.

Ako se domena mijenja, pokrenite [Domain Migration tool](/guide-migrations.html#migrating-domains) nakon importa. Ako se URL-ovi pojedinih stranica mijenjaju, možete [migrirati svaku stranicu](/guide-migrations.html#migrating-pages) sa stare URL adrese na novu.

Za masovne migracije stranica, primjerice uklanjanje domene iz vrijednosti koju prosljeđujete u polje [urlId](/guide-customizations-and-configuration.html#url-id) widgeta za komentare, [otvorite support ticket](https://fastcomments.com/auth/my-account/help) i mi ćemo to riješiti za vas.

### Prije prelaska

Import možete pokrenuti koliko god puta želite. Ponovni import iste datoteke [ne stvara duplikate](/guide-migrations.html#importing-data), pa možete jednom importirati kako biste testirali novu lokaciju, a zatim ponovno importirati s najnovijim komentarima neposredno prije prelaska.

Za izvozne datoteke veće od 1GB, [obratite se podršci](https://fastcomments.com/auth/my-account/help).

Za dodavanje FastCommentsa na vašu novu lokaciju pogledajte [Installation guide](/guide-installation.html).