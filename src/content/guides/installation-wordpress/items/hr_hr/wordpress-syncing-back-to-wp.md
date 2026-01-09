Po zadanoj postavci, FastComments sinkronizira se natrag na vašu WordPress stranicu dnevno. Ovo je isključivo u svrhu sigurnosne kopije kako biste i dalje posjedovali kopiju podataka, i za dodatke
koji se mogu osloniti na nju.

Ovo se ne događa odmah za svaki spremljeni komentar zbog činjenice da neke stranice mogu podnijeti veliki čitateljski promet, ali njihove baze podataka nisu uvijek sposobne podnijeti veliki zapisni promet (otuda prebacivanje ovog posla na FastComments).

Raspored sinkronizacije natrag na WordPress može se prilagoditi instalacijom dodatka. Preporučujemo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Instalirajte WP Crontrol
2. Idite na `Settings -> Cron Schedules`.
3. Otvorite karticu `Cron Events`.
4. Potražite `fastcomments_cron_hook`.
5. Uredite događaj. Možete konfigurirati hook da se izvršava satno, dvaput dnevno, dnevno (zadano), ili jednom tjedno.

Sinkronizacija natrag na WordPress također se može izvršiti bilo kada ručno odlaskom na nadzornu ploču FastComments dodatka i odabirom `Manually Sync`. Imat ćete
mogućnost sinkronizirati natrag na vašu WP instalaciju, ili ponovno učitati vaše WP komentare na FastComments poslužitelje.