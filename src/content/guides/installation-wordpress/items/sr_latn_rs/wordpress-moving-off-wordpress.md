Ako prebacujete svoj sajt sa WordPress-a i želite FastComments na novom sajtu, ne treba vam WordPress dodatak. Izvezite svoje komentare iz
WordPress-a, a zatim otpremite datoteku na [Import page](https://fastcomments.com/auth/my-account/manage-data/import) u FastComments kontrolnoj tabli.

Podržavamo dva formata WordPress izvoza.

### WordPress XML (Preporučeno)

Ovo je datoteka iz WordPress-ovog ugrađenog alata za izvoz, tako da nije potreban dodatni dodatak.

1. U svom WordPress admin panelu idite na `Tools -> Export`.
2. Izaberite `All content` i kliknite `Download Export File`.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), izaberite `WordPress (.xml)` i otpremite datoteku.

Svaki komentar je vezan za URL objave na kojoj je ostavljen, koji je već u datoteci.

Import zadržava ime autora, email i veb-sajt, datum, sadržaj, grananje odgovora i to da li je komentar odobren. Avatari komentatora se
prenose sa Gravatara. Glasovi nisu deo ovog formata.

### WordPress CSV

Ovo je datoteka iz [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Instalirajte dodatak u svom WordPress admin panelu i izvezite komentare kao CSV.
2. Zamenite svaku vrednost `comment_post_ID` URL-om objave.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), izaberite `WordPress (.csv)` i otpremite datoteku.

Svaki komentar je vezan za kolonu `comment_post_ID`. WordPress popunjava ovu kolonu ID-om objave, a vaš novi sajt nema WordPress ID-jeve objava,
pa korak 2 zamenjuje tu vrednost URL-om.

Import zadržava ime autora, email i veb-sajt, datum, sadržaj, grananje odgovora i to da li je komentar odobren. Avatari komentatora se
prenose sa Gravatara. Takođe zadržava WordPress-ovu spam oznaku, kao i wpDiscuz sviđanja i nesviđanja kada ih datoteka sadrži.

### Uparivanje komentara sa vašim novim stranicama

Ako vaš novi sajt zadržava iste URL-ove kao vaš WordPress sajt, komentari se pojavljuju na odgovarajućim stranicama bez dodatnog podešavanja.

Ako se domen menja, pokrenite [Domain Migration tool](/guide-migrations.html#migrating-domains) nakon importa. Ako se URL-ovi pojedinačnih stranica menjaju, možete
[migrirati svaku stranicu](/guide-migrations.html#migrating-pages) sa starog URL-a na novi.

Za masovne migracije stranica, na primer uklanjanje domena iz vrednosti koju prosleđujete polju [urlId](/guide-customizations-and-configuration.html#url-id)
vidžeta za komentare, [otvorite tiket za podršku](https://fastcomments.com/auth/my-account/help) i mi ćemo to obaviti za vas.

### Pre prebacivanja

Import možete pokrenuti koliko god puta želite. Ponovni import iste datoteke [ne stvara duplikate](/guide-migrations.html#importing-data), tako da možete
importovati jednom da testirate novi sajt, a zatim ponovo importovati sa najnovijim komentarima neposredno pre prebacivanja.

Za izvozne datoteke veće od 1GB, [obratite se podršci](https://fastcomments.com/auth/my-account/help).

Da dodate FastComments na svoj novi sajt, pogledajte [Installation guide](/guide-installation.html).