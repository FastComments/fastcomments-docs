Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima zmogljiv mehanizem uvoza, ki temelji na uporabniškem vmesniku. Po namestitvi vtičnika vas bo vodil skozi povezovanje vaše namestitve WordPressa s FastComments in kopiranje obstoječih podatkov komentarjev.

**To se izvede brez ročnega kopiranja ali prenosa česarkoli.**

Postopek migracije bo prikazan v uporabniškem vmesniku med migracijo. Večina migracij traja le nekaj minut.

Mehanizem je zasnovan tako, da med migracijo ne obremeni preveč vaše namestitve WordPress.

Če selite svoje spletno mesto iz WordPressa, lahko namesto uporabe vtičnika uvozite izvoz WordPress XML ali CSV. Glej [Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Da bi avtomatizirana nastavitev WordPressa delovala, moramo izvajati klice na vašo namestitev WordPress. Požarni zidovi, kot je Cloudflare, nas lahko blokirajo in povzročijo, da integracija ne uspe. V takih primerih vam [lahko zagotovimo](https://fastcomments.com/auth/my-account/help) nabor IP-jev, ki jih je treba vpisati na beli seznam za integracijo.

### Data Ownership

V primeru naše WordPress migracije se vsi novi ali posodobljeni podatki komentarjev samodejno sinhronizirajo nazaj v vašo namestitev WordPress v ozadju. To pomeni, da medtem ko komentarje streže FastComments sam, da zmanjša obremenitev vaše namestitve WordPress, mi **prav tako** shranimo komentarje v vašo bazo podatkov kot varnostno kopijo. To tudi pomeni, da če želite preiti stran od FastComments, so vaši podatki že migrirani in posodobljeni.