Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan mehanizam uvoza temeljen na UI-ju. Nakon instalacije dodatka, vodit će vas kroz povezivanje vaše WordPress instalacije s FastComments i kopiranje vaših postojećih podataka o komentarima.

**Ovo se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije bit će vam prikazan putem UI-a tijekom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran da ne opterećuje previše vašu WordPress instalaciju tijekom migracije.

Ako premještate svoju stranicu s WordPressa, možete uvesti WordPress XML ili CSV izvoz umjesto korištenja dodatka. Pogledajte [Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Kako bi automatizirano postavljanje WordPressa funkcioniralo, moramo slati zahtjeve vašoj WordPress instalaciji. Vatrozidi poput Cloudflare-a mogu nas blokirati i uzrokovati neuspjeh integracije. U takvim slučajevima, [možemo vam](https://fastcomments.com/auth/my-account/help) pružiti skup IP adresa koje treba staviti na bijelu listu za integraciju.

### Data Ownership

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima automatski se sinkroniziraju natrag u vašu WordPress instalaciju u pozadini. To znači da, iako FastComments sam isporučuje komentare kako bi smanjio opterećenje vaše WordPress instalacije, mi **također** spremamo ih u vašu bazu podataka kao sigurnosnu kopiju. To također znači da, ako želite preći s FastComments, vaši podaci su već migrirani i ažurirani.