Po defaultu agent se pokreće **odmah** nakon što se okidač aktivira. Polje **Delay before running** na formi za izmenu menja to: platforma stavlja okidač u red i pokreće agenta u zakazano vreme.

### Kada koristiti kašnjenje

- **Flag-threshold triggers** - flagovi često stižu u naletima. Kašnjenje od 10–30 minuta omogućava da se situacija smiri, tako da agent deluje na osnovu konačnog broja flagova, a ne trenutka dolaska.
- **Vote-threshold triggers** - isti princip, posebno kod koordinisanih downvote brigada.
- **Thread summarization** - the [Thread Summarizer template](#template-thread-summarizer) podrazumevano ima kašnjenje od 30 minuta, pa sumira razgovor koji je imao vremena da se razvije, a ne nit sa dve replike.
- **Cool-down / re-evaluation** - "24 hours after a comment is locked, consider whether to unlock it."

### Konfiguracija

- **Field**: Delay before running.
- **Range**: 0 do 2,592,000 sekundi (30 dana).
- **Units**: Sekunde, minute, sati ili dani.

### Idempotentnost

Odloženi red ne uklanja duplikate okidača. Dva flaga koja stignu sa razmakom od 1 sekunde na agentu sa 30-minutnim kašnjenjem će oba zakazati pokretanje 30 minuta kasnije, i agent će se pokrenuti **dvaput**, oba puta nad (uglavnom) istim kontekstom. Ako želite semantiku najviše-jedno-pokretanje-po-prozoru, agent mora to da obezbedi — obično tako što će pri prvom pokretanju upisati [memory note](#tools-overview) i proveriti ga pri narednim pokretanjima.

### Napomena o troškovima

Odloženi okidači se beleže **pre** nego što se pokrenu. Naleti okidača na agentu sa velikim kašnjenjem mogu se nagomilati u redu bez trošenja tokena; trošak se plaća tek kada ih cron rasporedi za izvršenje. Koristite [Run History](#run-history) i [Drop Reasons](#drop-reasons) da vidite koliko često odloženi okidači zaista budu izvršeni naspram toga koliko ih se u vreme pokretanja odbaci iz budžetskih razloga.

### Reprodukcija ne poštuje kašnjenje

Funkcija [Test Runs (Replays)](#test-runs-replays) pokreće agenta odmah nad istorijskim komentarima — ne čeka konfigurisano kašnjenje. Smatrajte to karakteristikom: replays služe za pregled šta bi agent **uradio** u datom kontekstu, a ne za reprodukciju rasporeda u realnom vremenu.