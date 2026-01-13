Sve promjene na objektu Comment u sustavu pokreću događaj koji završi u redu.

Početni webhook događaj obično se šalje unutar šest sekundi od nastanka izvora događaja.

Ovaj red možete nadzirati u administratorskom sučelju Webhooks u slučaju da vaš API padne.

Ako zahtjev prema vašem API-ju ne uspije, ponovno ćemo ga staviti u red prema rasporedu.

Taj raspored je `1 Minute * the retry count`. Ako poziv ne uspije jednom, pokušat će ponovno za
minutu. Ako ne uspije dvaput, tada će pričekati dvije minute, i tako dalje. Ovo je kako bismo
izbjegli preopterećenje vašeg API-ja ako pada iz razloga povezanih s opterećenjem.

Webhooks se mogu otkazati sa [stranice zapisnika](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).