**Doradi Upit** je tijek rada za uređivanje agenta [početnog prompta](#personality-prompt) kao odgovor na određene odluke s kojima se ne slažete. Pokreće se iz [pretinca odobrenja](#approval-workflow).

### Kada ga koristiti

Kada opetovano odbacujete istu vrstu odobrenja — "agent stalno želi zabraniti ljude zbog korištenja oštrog jezika bez cilja" — prompt agenta je poluga za rješavanje toga. Doradi Upit je vođeni način za:

1. Odaberite konkretno odobrenje koje predstavlja lošu odluku.
2. Uredite prompt s potpunim kontekstom onoga što je agent učinio i zašto.
3. Spremite novi prompt u agenta.

Rezultat je agent koji, ubuduće, vjerojatno neće donositi istu odluku.

### Pokretanje tijeka

Iz pretinca odobrenja na `/auth/my-account/ai-agent-approvals`:

1. Otvorite **odbijeno** odobrenje. Ruta strogo odbija sve osim REJECTED - pending i execution-failed odobrenja nisu podobna.
2. Kliknite **Doradi prompt**.

Nalazite se na sučelju za doradu prompta na `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Što stranica prikazuje

- **Odobrenje** - agentov `toolName` i `justification` za odbijenu odluku (ovdje nije prikazan cijeli transkript LLM-a).
- **Trenutni prompt** - spremljeni [početni prompt](#personality-prompt) agenta.
- **Polje za povratne informacije** - upisujete **feedback** koji opisuje što bi se trebalo promijeniti (do 2000 znakova). LLM zatim generira predloženi novi prompt na temelju vašeg feedbacka.
- **Ujedinjeni inline diff** - jedinstveni inline diff između trenutnog i predloženog prompta (crveno za uklonjeno, zeleno za dodano).

Kontekst odobrenja ostaje prikvačen na vrhu kako biste se mogli stalno pozivati na "slučaj koji popravljam" dok uređujete.

### Spremi

Spremanje ažurira polje agenta `initialPrompt`. Prošla izvođenja (i prošla odobrenja) se ne pokreću retroaktivno - novi prompt utječe samo na buduće okidače. Ako želite provjeriti rješava li novi prompt problem, pokrenite [test run / replay](#test-runs-replays) za posljednjih 7 dana i provjerite bi li novi prompt i dalje proizveo odbijeno odobrenje.

### Što tijek ne radi

- Ne uređuje **community guidelines** - to polje ima vlastiti uređivač na glavnom obrascu za uređivanje agenta.
- Ne uređuje **triggers**, **allowed tools**, ili **approval gating** - oni ostaju na glavnom obrascu za uređivanje.
- Ne verzionira prompt s mogućnošću povratka. Prethodni prompt nije pohranjen u zasebnu kolekciju povijesti. Ako trebate vratiti promjene, kopirajte trenutni prompt u vlastiti sustav praćenja prije uređivanja.

### Zašto upariti doradu s replayom

Uređivanje prompta bez testiranja rezultata je zasnovano na vjeri. Preporučeni ciklus:

1. Odbacite odobrenje.
2. Doradite prompt.
3. Pokrenite [test run](#test-runs-replays) za posljednjih 7 dana.
4. Pogledajte karticu **Deltas**. Je li novi prompt pomaknuo lošu odluku iz "would do" u "would not do"? Je li slučajno pomaknuo i dobre odluke izvan ranga?
5. Ponavljajte.

Tri ili četiri ciklusa dorade + replaya obično su dovoljna da se dobije stabilan prompt za agenta za moderaciju.

### Izravna alternativa za uređivanje

Ne morate koristiti Doradi Upit - možete također jednostavno urediti agenta na glavnom obrascu za uređivanje. Jedina prednost Doradi Upit je što prikvači određeni neuspjeli slučaj pa ne izgubite iz vida što popravljate.

---