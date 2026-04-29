**Doradi prompt** je workflow za uređivanje agenta [initial prompt](#personality-prompt) kao odgovor na specifične odluke sa kojima se ne slažete. Pokreće se iz [approvals inbox](#approval-workflow).

### Kada ga koristiti

Kada stalno odbijate istu vrstu odobrenja - "agent stalno želi da banuje ljude zbog upotrebe grubog jezika bez ciljanog subjekta" - prompt agenta je poluga kojom to možete ispraviti. Doradi prompt je vođeni način da:

1. Izaberete konkretno odobrenje koje predstavlja lošu odluku.
2. Izmenite prompt sa potpunim kontekstom šta je agent uradio i zašto.
3. Sačuvate novi prompt za agenta.

Rezultat je agent koji, ubuduće, verovatno neće doneti istu odluku.

### Pokretanje toka

Iz approvals inbox-a na `/auth/my-account/ai-agent-approvals`:

1. Otvorite **odbijeno** odobrenje. Ruta strogo odbacuje sve osim REJECTED - pending i execution-failed odobrenja nisu prihvatljiva.
2. Kliknite **Doradi prompt**.

Dolazite na korisnički interfejs za doradu prompta na `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Šta stranica prikazuje

- **Odobrenje** - agentov `toolName` i `justification` za odbijenu odluku (puni LLM transkript se ovde ne prikazuje).
- **Trenutni prompt** - sačuvani [initial prompt](#personality-prompt) agenta.
- **Polje za povratnu informaciju** - unesete **povratnu informaciju** u kojoj opisujete šta bi trebalo da se promeni (do 2000 karaktera). LLM zatim generiše predloženi novi prompt na osnovu vaše povratne informacije.
- **Ujednačeni inline diff** - jedinstveni inline diff između trenutnog i predloženog prompta (crveno za uklonjeno, zeleno za dodato).

Kontekst odobrenja ostaje prikvačen na vrhu tako da se možete stalno pozivati na "slučaj koji popravljam" dok uređujete.

### Sačuvaj

Sačuvavanje ažurira polje agenta `initialPrompt`. Prošli pokreti (i prošla odobrenja) se ne pokreću retroaktivno - novi prompt utiče samo na buduće okidače. Ako želite da verifikujete da novi prompt rešava problem, pokrenite [test run / replay](#test-runs-replays) za poslednjih 7 dana i proverite da li bi novi prompt i dalje proizveo odbijeno odobrenje.

### Šta ovaj tok ne radi

- Ne uređuje **community guidelines** - to polje ima svoj sopstveni editor na glavnom obrascu za uređivanje agenta.
- Ne uređuje **triggers**, **allowed tools**, ili **approval gating** - oni ostaju na glavnom obrascu za uređivanje.
- Ne verzionira prompt sa rollback opcijom. Prethodni prompt nije sačuvan u posebnoj kolekciji istorije. Ako treba da vratite promene, kopirajte trenutni prompt u sopstveni sistem za praćenje pre uređivanja.

### Zašto upariti doradu sa replay-om

Uređivanje prompta bez testiranja rezultata je stvar vere. Preporučeni ciklus:

1. Odbijte odobrenje.
2. Doradite prompt.
3. Pokrenite [test run](#test-runs-replays) za poslednjih 7 dana.
4. Pogledajte karticu **Deltas**. Da li je novi prompt pomerio lošu odluku iz "would do" u "would not do"? Da li je slučajno pomerio i dobre odluke van opsega?
5. Iterirajte.

Tri ili četiri ciklusa dorade + replay-a obično su dovoljna da se dobije stabilan prompt za agenciju za moderaciju.

### Direktna alternativa za uređivanje

Ne morate koristiti Doradi prompt - možete i jednostavno urediti agenta na glavnom obrascu za uređivanje. Jedina prednost Doradi prompta je što prikvači konkretan neuspešni slučaj tako da ne izgubite iz vida šta pokušavate da popravite.