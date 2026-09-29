Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan UI‑baziran mehanizam za uvoz. Nakon instalacije plugina,
on će vas voditi kroz povezivanje vaše WordPress instalacije sa FastComments i kopiranje vaših postojećih podataka o komentarima.

**Ovo se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije biće prikazan putem UI‑ja tokom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran da ne opterećuje previše vašu WordPress instalaciju tokom migracije.

Ako premeštate svoj sajt sa WordPress‑a, možete uvesti WordPress XML ili CSV izvoz umesto korišćenja plugina. Pogledajte
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Da bi automatizovano podešavanje WordPress‑a radilo, moramo da pravimo pozive vašoj WordPress instalaciji.
Firewall‑i poput Cloudflare‑a mogu nas blokirati i uzrokovati neuspeh integracije. U takvim slučajevima, [možemo vam
pružiti](https://fastcomments.com/auth/my-account/help) skup IP adresa koje treba whitelist‑ovati za integraciju.

### Data Ownership

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima automatski se sinhronizuju nazad u vašu WordPress instalaciju
u pozadini. To znači da, iako komentare servira sam FastComments kako bi smanjio opterećenje vaše WordPress instalacije,
mi **takođe** čuvamo ih u vašoj bazi podataka kao rezervu. Ovo takođe znači da, ako želite da pređete sa FastComments‑a, vaši podaci su
već migrirani i ažurirani.