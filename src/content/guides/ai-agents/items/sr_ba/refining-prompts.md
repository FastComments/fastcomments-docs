**Doradi prompt** je tok rada za uređivanje [initial prompt](#personality-prompt) agenta kao odgovor na određene odluke sa kojima se ne slažete. Pokreće se iz [approvals inbox](#approval-workflow).

### Kada ga koristiti

Kada stalno odbacujete isti tip odobrenja — „agent stalno želi da banuje ljude zbog upotrebe grubog jezika bez cilja“ — prompt agenta je poluga za ispravljanje toga. Refine Prompt je vođeni način da:

1. Odaberete konkretno odobrenje koje predstavlja lošu odluku.
2. Izmijenite prompt uz puni kontekst onoga što je agent uradio i zašto.
3. Sačuvate novi prompt za agenta.

Rezultat je agent koji, ubuduće, vjerovatno neće donositi istu odluku.

### Pokretanje toka

Iz approvals inbox na `/auth/my-account/ai-agent-approvals`:

1. Otvorite **rejected** odobrenje. Ruta odbacuje sve osim REJECTED - pending i execution-failed odobrenja nisu prihvatljiva.
2. Kliknite **Refine prompt**.

Dosjećete se na prompt-refine UI na `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Šta stranica prikazuje

- **The approval** - agentov `toolName` i `justification` za odbijenu odluku (cijeli LLM transkript ovdje nije prikazan).
- **The current prompt** - sačuvani [initial prompt](#personality-prompt) agenta.
- **A feedback input** - unesete **feedback** koji opisuje šta treba promijeniti (do 2000 znakova). LLM zatim generiše predloženi novi prompt na osnovu vašeg feedbacka.
- **Unified inline diff** - jedan inline diff između trenutnog i predloženog prompta (crveno za uklonjeno, zeleno za dodato).

Kontekst odobrenja ostaje prikvačen na vrhu tako da možete stalno upućivati na „slučaj koji popravljam“ dok uređujete.

### Sačuvaj

Sačuvavanje ažurira polje `initialPrompt` agenta. Prošli pokreti (i prošla odobrenja) se ne pokreću retroaktivno — novi prompt utiče samo na buduće okidače. Ako želite provjeriti da li novi prompt rješava problem, pokrenite [probni pokret / repriza](#test-runs-replays) za posljednjih 7 dana i provjerite da li bi novi prompt i dalje proizveo odbijeno odobrenje.

### Šta tok ne radi

- Ne uređuje **community guidelines** — to polje ima svoj editor na glavnom obrascu za uređivanje agenta.
- Ne uređuje **triggers**, **allowed tools**, ili **approval gating** — oni ostaju na glavnom obrascu za uređivanje.
- Ne verzionira prompt sa rollbackom. Prethodni prompt nije pohranjen u posebnoj historijskoj kolekciji. Ako trebate vratiti promjenu, kopirajte trenutni prompt u svoj sistem za praćenje prije uređivanja.

### Zašto upariti refine sa replay

Uređivanje prompta bez testiranja rezultata je zasnovano na vjerovanju. Preporučeni ciklus:

1. Odbijte odobrenje.
2. Doradite prompt.
3. Pokrenite [probni pokret](#test-runs-replays) za posljednjih 7 dana.
4. Pogledajte karticu **Deltas**. Da li je novi prompt pomjerio lošu odluku iz „would do“ u „would not do“? Da li je slučajno pomjerio dobre odluke vani također?
5. Iterirajte.

Tri ili četiri ciklusa refine + replay obično su dovoljna da se dobije stabilan prompt za moderacijski agent.

### Direktna alternativa uređivanja

Ne morate koristiti Refine Prompt — možete jednostavno uređivati agenta na glavnom obrascu za uređivanje. Jedina prednost Refine Prompt je što prikvači konkretan neuspješan slučaj tako da ne izgubite iz vida šta popravljate.