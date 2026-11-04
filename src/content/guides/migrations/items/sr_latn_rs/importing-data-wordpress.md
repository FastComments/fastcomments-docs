Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima moćan mehanizam uvoza zasnovan na UI-ju. Nakon instalacije dodatka,
on će vas voditi kroz povezivanje vaše WordPress instalacije sa FastComments i kopiranje vaših postojećih podataka o komentarima.

**Ovo se radi bez ručnog kopiranja ili preuzimanja bilo čega.**

Proces migracije biće vam prikazan putem UI-ja tokom migracije. Većina migracija traje samo nekoliko minuta.

Mehanizam je dizajniran tako da ne stavlja prekomjerno opterećenje na vašu WordPress instalaciju tokom migracije.

Ako premeštate svoj sajt sa WordPressa, umesto korišćenja dodatka možete uvesti WordPress XML ili CSV eksport. Pogledajte
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i firewallovi

Da bi automatsko WordPress podešavanje radilo, moramo da upućujemo pozive ka vašoj WordPress instalaciji.
Firewallovi poput Cloudflarea mogu nas blokirati i prouzrokovati neuspeh integracije. U takvim slučajevima, [možemo vam
dati](https://fastcomments.com/auth/my-account/help) skup IP adresa koje treba da stavite na belu listu za integraciju.

### Vlasništvo nad podacima

U slučaju naše WordPress migracije, svi novi ili ažurirani podaci o komentarima se automatski sinhronizuju nazad ka vašoj WordPress instalaciji
u pozadini. To znači da, dok komentare servira sam FastComments kako bi smanjio opterećenje vašeg WordPress deploymenta,
mi ih **takođe** čuvamo u vašoj bazi podataka kao rezervnu kopiju. To takođe znači da, ako poželite da pređete sa FastCommentsa, vaši podaci su
već migrirani i ažurni.