Podrazumevano, FastComments se sinhronizuje nazad na vaš WordPress sajt svakodnevno. Ovo je isključivo u svrhu pravljenja rezervne kopije kako biste i dalje posedovali kopiju podataka, i za dodatke (plugin-e) koji mogu da zavise od toga.

Ovo se ne dešava odmah pri čuvanju svakog komentara, jer zbog prirode nekih sajtova koji mogu da podnesu veliki broj čitanja, njihove baze podataka nisu uvek u stanju da podnesu veliki broj upisa (otuda prebacivanje ovog posla na FastComments).

Raspored sinhronizacije nazad na WordPress može se prilagoditi instaliranjem dodatka. Preporučujemo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Koraci:

1. Instalirajte WP Crontrol
2. Idite na `Settings -> Cron Schedules`.
3. Idite na tab `Cron Events`.
4. Pretražite `fastcomments_cron_hook`.
5. Izmenite događaj. Možete podesiti da se hook izvršava na svaki sat, dva puta dnevno, svakodnevno (podrazumevano) ili jednom nedeljno.

Sinhronizacija nazad na WordPress takođe može biti izvršena ručno u bilo kom trenutku tako što ćete otići na kontrolnu tablu FastComments dodatka i izabrati `Manually Sync`. Imaćete opciju da se sinhronizujete nazad na vašu WP instalaciju, ili da ponovo otpremite vaše WP komentare na FastComments servere.