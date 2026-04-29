**Probno pokretanje** je sigurnosni način u kojem svaki novi agent započinje. Agent se izvršava od početka do kraja osim dijela gdje dodiruje vašu zajednicu.

### Što se izvršava u probnom pokretanju

- Okidači se aktiviraju normalno.
- Agentov prompt, [smjernice zajednice](#community-guidelines) i [kontekst](#context-options) se sastavljaju.
- Poziva se LLM.
- Model odabire pozive alata i daje obrazloženja + ocjene pouzdanosti.
- Izvršavanje se bilježi oznakom **Probno pokretanje** tako da je jasno odvojeno od živih izvršavanja.

### Što se ne izvršava u probnom pokretanju

- Niti jedan komentar se ne objavljuje, niti se glas daje, niti se komentar prikvačuje/otkvačuje/zaključava/otključava.
- Niti jedan komentar nije označen kao neželjena pošta, odobren ili pregledan.
- Nijedan korisnik nije zabranjen, upozoren ili nagrađen značkom.
- Nijedna e-pošta nije poslana.
- Ništa se ne zapisuje u memoriju. (Da — uključujući memoriju. Agentu u probnom pokretanju nije dopušteno zatrovati zajednički skup memorije hipotetičkim odlukama.)
- Nijedan webhook se ne aktivira za radnje alata. (Webhookovi na razini okidača `trigger.succeeded` / `trigger.failed` i dalje se aktiviraju, a u payloadu je `wasDryRun: true`. Vidi [Webhook Payloads](#webhook-payloads).)

### Što to košta

Probno pokretanje izvršava **isti LLM poziv** koji bi izvršavanje u stanju **Omogućeno** izvršilo. Tokeni se naplaćuju, primjenjuju se [ograničenja budžeta](#budgets-overview), a izvršavanja se uračunavaju u dnevna/mjesečna ograničenja po agentu i po najmoprimcu.

Taj trošak je cijena dobivanja vjernog pregleda. Način "preskoči LLM poziv" ne bi vam dao nikakav signal o tome kako bi se agent ponašao.

### Čitanje rezultata probnog pokretanja

U [Run History](#run-history), izvršavanja u probnom pokretanju označena su oznakom **Probno pokretanje** u stupcu statusa. Radnje unutar svakog izvršavanja izgledaju identično kao i žive radnje - isti naziv alata, isti argumenti, isto obrazloženje i ista ocjena pouzdanosti - osim što se nijedna od njih nije dogodila.

Na [Analytics stranici](#analytics-page) prikazuje se usporedba "probno pokretanje vs živo" izvršavanja po mjesecima kako biste vidjeli koliko je vaših tokena potrošeno na promatranje.

### Prebacivanje iz probnog pokretanja

Uredite agenta i promijenite **Status** u **Omogućeno**. Sljedeći okidač će se izvršiti uživo.

Također možete prebaciti i u suprotnom smjeru — iz **Omogućeno** natrag u **Probno pokretanje** — ako agent počne raditi stvari koje vam se ne sviđaju. Nema kazne.

### Reprodukcije prisiljavaju probno pokretanje

Značajka [Test Runs (Replays)](#test-runs-replays) pokreće agenta protiv povijesnih komentara **uvijek u probnom pokretanju**, bez obzira na spremljeni status agenta. Reprodukcije ne mogu poduzimati stvarne radnje na prošlim komentarima. To je po dizajnu - reprodukcija je alat za pregled, ne alat za moderiranje.

---