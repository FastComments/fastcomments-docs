Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan mehanizam uvoza temeljen na korisničkom sučelju. Nakon instalacije dodatka,
on će vas voditi kroz povezivanje vaše WordPress instalacije s FastComments i kopiranje vaših postojećih podataka o komentarima.

**To se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije bit će vam prikazan putem korisničkog sučelja tijekom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran tako da ne stavlja pretjerano opterećenje na vašu WordPress instalaciju tijekom migracije.

Ako premještate svoju stranicu s WordPressa, umjesto dodatka možete uvesti WordPress XML ili CSV izvoz. Pogledajte
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i vatrozidi

Kako bi automatska WordPress postava radila, moramo upućivati pozive na vašu WordPress instalaciju.
Vatrozidi poput Cloudflarea mogu nas blokirati i uzrokovati neuspjeh integracije. U takvim slučajevima, [možemo vam dati](https://fastcomments.com/auth/my-account/help) skup IP adresa za whitelisting radi integracije.

### Vlasništvo nad podacima

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima automatski se sinkroniziraju natrag na vašu WordPress instalaciju
u pozadini. To znači da, dok komentare poslužuje sam FastComments kako bi smanjio opterećenje vaše WordPress postave,
mi ih **također** spremamo u vašu bazu podataka kao sigurnosnu kopiju. To također znači da, ako želite prijeći s FastCommentsa, vaši su podaci
već migrirani i ažurirani.