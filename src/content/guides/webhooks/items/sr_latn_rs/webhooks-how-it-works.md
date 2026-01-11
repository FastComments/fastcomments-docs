Sve promene Comment objekta u sistemu pokreću događaj koji se na kraju nađe u redu.

Početni webhook događaj se obično šalje u roku od šest sekundi od nastanka izvora događaja.

Možete pratiti ovaj red u Webhooks adminu u slučaju da vaš API bude nedostupan.

Ako zahtev prema vašem API-ju ne uspe, ponovo ćemo ga staviti u red po rasporedu.

Taj raspored je `1 Minute * the retry count`. Ako poziv ne uspe jednom, pokušaće ponovo za minut. Ako ne uspe dvaput, onda će sačekati dva minuta, i tako dalje. To je da ne bismo preopteretili vaš API ako on pada zbog razloga povezanih sa opterećenjem.

Webhooks se mogu otkazati sa [stranice logova](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).