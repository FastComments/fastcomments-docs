Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan mehanizam uvoza zasnovan na korisničkom interfejsu. Nakon instalacije dodatka,
bićete vođeni kroz povezivanje vaše WordPress instalacije sa FastComments i kopiranje vaših postojećih podataka o komentarima.

**Ovo se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije biće vam prikazan putem korisničkog interfejsa tokom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran tako da ne stavlja prekomerno opterećenje na vašu WordPress instalaciju tokom migracije.

Ako premeštate svoj sajt sa WordPress-a, umesto korišćenja dodatka možete uvesti WordPress XML ili CSV eksport. Pogledajte
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i zaštitni zidovi

Da bi automatsko WordPress podešavanje funkcionisalo, moramo upućivati pozive ka vašoj WordPress instalaciji.
Zaštitni zidovi poput Cloudflare-a mogu nas blokirati i uzrokovati neuspeh integracije. U takvim slučajevima, [možemo vam pružiti](https://fastcomments.com/auth/my-account/help) skup IP adresa koje treba da stavite na belu listu za integraciju.

### Vlasništvo nad podacima

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima se automatski sinhronizuju nazad ka vašoj WordPress instalaciji
u pozadini. To znači da, dok komentare servira sam FastComments kako bi smanjio opterećenje vašeg WordPress raspoređivanja,
mi ih **takođe** čuvamo u vašoj bazi podataka kao rezervnu kopiju. Ovo takođe znači da, ako poželite da pređete sa FastComments-a, vaši podaci su
već migrirani i ažurni.