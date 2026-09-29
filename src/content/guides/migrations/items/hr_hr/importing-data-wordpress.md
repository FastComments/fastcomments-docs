Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan UI‑baziran mehanizam uvoza. Nakon instalacije dodatka,
on će vas voditi kroz povezivanje vaše WordPress instalacije s FastComments i kopiranje vaših postojećih podataka o komentarima.

**Ovo se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije bit će vam prikazan putem UI‑a tijekom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran da ne opterećuje previše vašu WordPress instalaciju tijekom migracije.

Ako premještate svoju stranicu s WordPressa, možete uvesti WordPress XML ili CSV izvoz umjesto korištenja dodatka. Pogledajte
[Premještanje vaših komentara na novu stranicu](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i FireWalls

Kako bi automatizirano postavljanje WordPressa funkcioniralo, moramo izvršavati pozive prema vašoj WordPress instalaciji.
Vatrozidi poput Cloudflarea mogu nas blokirati i uzrokovati neuspjeh integracije. U takvim slučajevima, [možemo vam pružiti](https://fastcomments.com/auth/my-account/help) skup IP adresa za bijelu listu za integraciju.

### Vlasništvo podataka

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima automatski se sinkroniziraju natrag u vašu WordPress instalaciju u pozadini.
To znači da, iako komentare poslužuje FastComments kako bi smanjio opterećenje vaše WordPress implementacije,
**također** ih spremamo u vašu bazu podataka kao sigurnosnu kopiju. To također znači da, ako želite preći s FastComments, vaši podaci su već migrirani i ažurirani.