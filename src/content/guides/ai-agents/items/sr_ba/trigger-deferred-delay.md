Po defaultu agent se pokreće **odmah** nakon što se njegov okidač aktivira. Polje **Delay before running** na formi za uređivanje to mijenja: platforma stavlja okidač u red i pokreće agenta u zakazano vrijeme.

### When to use a delay

- **Flag-threshold triggers** - zastavice često stižu u talasima. Odgoda od 10-30 minuta dopušta da se slika stabilizuje tako da agent djeluje na konačni broj zastavica, a ne na trenutak dolaska.
- **Vote-threshold triggers** - ista logika, naročito kod brigada za negativno glasanje.
- **Thread summarization** - [Predložak za sažimanje niti](#template-thread-summarizer) po defaultu ima odgodu od 30 minuta kako bi sažimao razgovor koji je imao vremena da se razvije, a ne nit koja je tek s dva odgovora.
- **Cool-down / re-evaluation** - „24 sata nakon što je komentar zaključan, razmotriti da li ga otključati.“

### Configuration

- **Field**: Delay before running.
- **Range**: 0 to 2,592,000 seconds (30 days).
- **Units**: Sekunde, minute, sati, ili dani.

### Idempotence

Odgođeni red ne uklanja duplikate okidača. Dvije zastavice koje stignu sa razmakom od 1 sekunde na agentu sa odgodom od 30 minuta obje će zakazati pokretanje 30 minuta kasnije, i agent će se pokrenuti **dvaput**, oba puta protiv (pretežno) istog konteksta. Ako želite semantiku „najviše jedno pokretanje po prozoru“, agent mora to sam sprovesti - obično tako što će pri prvom pokretanju zapisati [memorijsku bilješku](#tools-overview) i provjeriti je pri naknadnim pokretanjima.

### Cost note

Odgođeni okidači se bilježe **prije** nego što se pokrenu. Talas okidača na agentu sa velikom odgodom može se nakupiti u redu bez trošenja tokena; trošak se plaća tek kada cron ih rasporedi. Koristite [Istorija pokretanja](#run-history) i [Razlozi odbacivanja](#drop-reasons) da vidite koliko često se odgođeni okidači zapravo izvrše naspram toga da su u toku izvršavanja odbijeni iz budžetskih razloga.

### Replay does not honor delay

Funkcija [Test pokretanja (reprodukcije)](#test-runs-replays) pokreće agenta odmah nad historijskim komentarima - ne čeka konfiguriranu odgodu. Smatrate to pogodnost: reprodukcije služe za pregled šta bi agent **urađao** s obzirom na kontekst, a ne za reprodukovanje stvarnog vremenskog rasporeda.