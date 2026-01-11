Sve promjene na Comment objektu u sistemu pokreću događaj koji završi u redu.

Početni webhook događaj obično se šalje u roku od šest sekundi od nastanka izvora događaja.

Možete pratiti ovaj red u Webhooks adminu u slučaju da vaš API prestane raditi.

Ako zahtjev ka vašem API-ju ne uspije, ponovo ćemo ga staviti u red po rasporedu.

Taj raspored je `1 Minute * the retry count`. Ako poziv ne uspije jednom, pokušaće ponovo za minutu. Ako ne uspije dva puta, onda će sačekati dvije minute, i tako dalje. Ovo je kako bismo izbjegli preopterećenje vašeg API-ja ako pada zbog razloga vezanih za opterećenje.

Webhooks se mogu otkazati sa [stranice logova](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).