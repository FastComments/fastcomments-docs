Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima zmogljiv mehanizem uvoza, ki temelji na uporabniškem vmesniku. Po namestitvi vtičnika vas bo vodil skozi povezovanje vaše namestitve WordPressa s FastComments in kopiranje obstoječih podatkov o komentarjih.

**To se izvede brez ročnega kopiranja ali prenosa česarkoli.**

Postopek migracije bo prikazan v uporabniškem vmesniku med migracijo. Večina migracij traja le nekaj minut.

Mehanizem je zasnovan tako, da med migracijo ne obremeni preveč vaše namestitve WordPressa.

Če selite svoje spletno mesto iz WordPressa, lahko namesto uporabe vtičnika uvozite izvoz WordPress XML ali CSV. Glejte
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Da bi avtomatizirana nastavitev WordPressa delovala, moramo izvajati klice na vašo namestitev WordPressa.
Požarni zidovi, kot je Cloudflare, nas lahko blokirajo in povzročijo, da integracija ne uspe. V takih primerih vam [lahko zagotovimo](https://fastcomments.com/auth/my-account/help) nabor IP-jev, ki jih je treba vpisati na beli seznam za integracijo.

### Data Ownership

V primeru naše WordPress migracije se vsi novi ali posodobljeni podatki o komentarjih samodejno sinhronizirajo nazaj v vašo namestitev WordPressa v ozadju. To pomeni, da medtem ko komentarje streže FastComments, da zmanjša obremenitev vaše WordPress namestitve,
**tudi** jih shranimo v vašo bazo podatkov kot varnostno kopijo. To tudi pomeni, da če želite preiti stran od FastComments, so vaši podatki že migrirani in posodobljeni.