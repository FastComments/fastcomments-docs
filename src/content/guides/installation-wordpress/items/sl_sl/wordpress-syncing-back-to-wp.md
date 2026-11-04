FastComments privzeto vsakodnevno sinhronizira nazaj na vaše WordPress spletno mesto. To je zgolj za namene varnostnega kopiranja, da ostanete lastnik kopije podatkov, ter za vtičnike,
ki so od njih lahko odvisni.

To se ne zgodi takoj ob vsakem shranjenem komentarju, saj pri nekaterih spletnih mestih, ki zmorejo obdelati veliko bralnega prometa, njihove namestitve baz podatkov niso vedno sposobne obdelati velikega pisalnega prometa (zato se to delo prenese na FastComments).

Urnik sinhronizacije nazaj na WordPress je mogoče prilagoditi z namestitvijo vtičnika. Priporočamo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Koraki:

1. Namestite WP Crontrol
2. Pojdite na `Settings -> Cron Schedules`.
3. Pojdite na zavihek `Cron Events`.
4. Poiščite `fastcomments_cron_hook`.
5. Uredite dogodek. Nastavite lahko, da se koda izvaja vsako uro, dvakrat dnevno, dnevno (privzeto) ali enkrat na teden.

Sinhronizacijo nazaj na WordPress je mogoče izvesti tudi kadar koli ročno, tako da odprete nadzorno ploščo vtičnika FastComments in izberete `Manually Sync`. Imeli boste
možnost sinhronizacije nazaj na vašo WP namestitev ali ponovnega nalaganja vaših WP komentarjev na strežnike FastComments.