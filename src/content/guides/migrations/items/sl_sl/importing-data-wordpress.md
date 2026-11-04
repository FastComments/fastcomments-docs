Naš [WordPress vtičnik](https://wordpress.org/plugins/fastcomments/) ima zmogljiv mehanizem uvoza z uporabniškim vmesnikom. Po namestitvi vtičnika
vas bo vodil skozi povezovanje vaše WordPress namestitve s FastComments in kopiranje vaših obstoječih podatkov o komentarjih.

**To se izvede brez ročnega kopiranja ali prenosov.**

Potek migracije vam bo prikazan prek uporabniškega vmesnika med migracijo. Večina migracij traja le nekaj minut.

Mehanizem je zasnovan tako, da med migracijo ne obremenjuje vaše WordPress namestitve prekomerno.

Če selite svoje spletno mesto iz WordPressa, lahko namesto vtičnika uvozite WordPress XML ali CSV izvoz. Glej
[Prenos komentarjev na novo spletno mesto](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare in požarni zidovi

Da bi samodejna nastavitev WordPressa delovala, moramo opravljati klice na vašo WordPress namestitev.
Požarni zidovi, kot je Cloudflare, nas lahko blokirajo in povzročijo neuspeh integracije. V takih primerih vam [lahko zagotovimo](https://fastcomments.com/auth/my-account/help) nabor IP naslovov za dodajanje na seznam dovoljenih za integracijo.

### Lastništvo podatkov

V primeru naše WordPress migracije se vsi novi ali posodobljeni podatki o komentarjih samodejno sinhronizirajo nazaj v vašo WordPress namestitev
v ozadju. To pomeni, da čeprav komentarje streže FastComments sam, da bi razbremenil vašo WordPress namestitev, jih **prav tako** shranjujemo v vašo podatkovno bazo kot varnostno kopijo. To pomeni tudi, da če se želite preklopiti s FastComments, so vaši podatki že migrirani in posodobljeni.