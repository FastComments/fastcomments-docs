Po zadanim postavkama agent se izvršava **odmah** nakon što se okidač aktivira. Polje **Delay before running** na obrascu za uređivanje to mijenja: platforma stavlja okidač u red čekanja i pokreće agenta u zakazano vrijeme.

### Kada koristiti odgodu

- **Okidači s pragom oznaka** - oznake često stižu u naletima. Odgoda od 10–30 minuta omogućuje da se situacija smiri pa agent djeluje na konačnom broju oznaka umjesto u trenutku dolaska.
- **Okidači po pragu glasova** - ista logika, posebno kod koordiniranog glasanja protiv.
- **Sažimanje teme** - [Thread Summarizer template](#template-thread-summarizer) podrazumijeva zadanu odgodu od 30 minuta kako bi sažeo razgovor koji je imao vremena da se razvije, a ne temu s dva odgovora.
- **Hlađenje / ponovno ocjenjivanje** - "24 sata nakon što je komentar zaključan, razmotrite treba li ga otključati."

### Konfiguracija

- **Field**: Delay before running.
- **Range**: 0 do 2.592.000 sekundi (30 dana).
- **Units**: Sekunde, minute, sati ili dani.

### Idempotencija

Odgođeni red čekanja ne uklanja duplicirane okidače. Dvije oznake koje stignu u razmaku od 1 sekunde na agentu s 30-minutnom odgodom obje će zakazati izvršavanje 30 minuta kasnije, i agent će se izvršiti **dvaput**, svaki put nad (uglavnom) istim kontekstom. Ako želite semantiku "najviše jedno izvršenje po prozoru", agent to mora provjeriti sam — obično pisanjem [memory note](#tools-overview) pri prvom izvršenju i provjerom istog pri narednim izvršavanjima.

### Napomena o troškovima

Odgođeni okidači se bilježe **prije** nego što se izvrše. Nalet okidača na agentu s velikom odgodom može se nagomilati u redu bez trošenja tokena; trošak se plaća tek kada cron rasporedi njihovo izvođenje. Koristite [Run History](#run-history) i [Drop Reasons](#drop-reasons) da vidite koliko se često odgođeni okidači zapravo izvršavaju u odnosu na to koliko ih se odbaci u vrijeme izvođenja zbog budžeta.

### Reprodukcija ne poštuje odgodu

Značajka [Test Runs (Replays)](#test-runs-replays) pokreće agenta odmah nad povijesnim komentarima — ne čeka konfiguriranu odgodu. Smatrajte to značajkom: reprodukcije služe za pregled onoga što bi agent **učinio** s obzirom na kontekst, a ne za reprodukciju rasporeda u stvarnom vremenu.