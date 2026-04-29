Agent ima jedan od tri statusa:

### Disabled

Agent je isključen. Nijedna okidač (trigger) se ne obrađuje i agent se ne pojavljuje u putanji dispečera. Njegova istorija izvršavanja, analitika i memorija ostaju — ako ga kasnije ponovo omogućite, istorijski podaci su i dalje tu.

Koristite `Disabled` kada:
- Želite da izuzmete agenta iz rotacije bez gubitka.
- Agent se nepravilno ponaša i morate ga odmah zaustaviti dok istražujete problem.
- Sezonski rotirate agente (npr. dočekivač koji radi samo tokom praznika).

### Dry Run - podrazumevano za nove agente

Agent izvršava kompletan tok — obrađuje okidače, poziva LLM, bira pozive alata, izračunava opravdanja i poverenje — ali **ne preduzima se nijedna stvarna akcija**. Svako izvršavanje se zapisuje sa oznakom **Dry Run** u [Istorija pokretanja](#run-history).

Koristite `Dry Run` kada:
- Novi agent je upravo iz kutije. Svaki starter template počinje u dry-run režimu.
- Izmenili ste prompt ili skup okidača i želite da vidite kako će se promena odraziti pre nego što je primenite.
- Izvršavate [test pokretanje / reprodukciju](#test-runs-replays) (reprodukcije prisiljavaju dry-run bez obzira na status agenta).

Platforma naplaćuje tokene i za dry-run izvršavanja — poziv LLM-a se i dalje dešava, samo se sporedni efekti preskaču. Ograničenja budžeta se primenjuju i na dry-run. Pogledajte [Pregled budžeta](#budgets-overview).

### Enabled

Agent preduzima stvarne akcije. Pozivi alata se izvršavaju — ili se stavljaju u red za [odobrenje](#approval-workflow) ako je akcija zaštićena.

Koristite `Enabled` nakon što izlaz iz dry-run izgleda ispravno.

### Promena statusa

Možete prebacivati između bilo koja dva statusa na formi za uređivanje. Prelazak iz `Dry Run` u `Enabled` ne ponovo izvršava retroaktivno akcije iz dry-run-a — one ostaju kao istorija dry-run-a. Novi okidači od tog trenutka pa nadalje se izvršavaju u realnom režimu.

Prelazak iz `Enabled` u `Disabled` usred izvršavanja **ne** prekida trenutno aktivno izvršavanje. Trenutno-izvršavajući okidač se dovršava (sa onim što je već započeo); sledeći okidač se odbacuje jer je agent sada `Disabled`.

### Status tokom problema sa naplatom

Ako naplata vašeg zakupca postane nevažeća, svi agenti su efektivno pauzirani bez obzira na sačuvani status — okidači se odbacuju sa `BILLING_INVALID` dok se naplata ne obnovi. Polje sačuvanog statusa se ne menja; dispečer jednostavno odbija da izvrši. Pogledajte [Planovi i podobnost](#plans-and-eligibility).