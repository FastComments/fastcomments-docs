Naš [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ima zmogljiv mehanizem uvoza, ki temelji na uporabniškem vmesniku. Po namestitvi vtičnika,
vas bo vodil skozi povezovanje vaše namestitve WordPress s FastComments in kopiranje obstoječih podatkov komentarjev.

**To se izvede brez ročnega kopiranja ali prenosa česarkoli.**

Postopek migracije vam bo prikazan prek uporabniškega vmesnika med migracijo. Večina migracij traja le nekaj minut.

Mehanizem je zasnovan tako, da med migracijo ne obremenjuje preveč vaše namestitve WordPress.

### CloudFlare & FireWalls

Da bi avtomatizirana nastavitev WordPressa delovala, moramo izvajati klice na vašo namestitev WordPress.
Požarni zidovi, kot je Cloudflare, nas lahko blokirajo in povzročijo, da integracija ne uspe. V takih primerih [vam lahko
zagotovimo](https://fastcomments.com/auth/my-account/help) nabor IP naslovov, ki jih je treba vpisati na beli seznam za integracijo.

### Data Ownership

V primeru naše WordPress migracije se vsi novi ali posodobljeni podatki komentarjev samodejno sinhronizirajo nazaj v vašo namestitev WordPress
v ozadju. To pomeni, da medtem ko komentarje streže sam FastComments, da zmanjša obremenitev vaše WordPress namestitve,
**tudi** jih shranimo v vašo bazo podatkov kot varnostno kopijo. To tudi pomeni, da če želite preiti stran od FastComments, so vaši podatki
že migrirani in posodobljeni.