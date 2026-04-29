Agent ima jedan od tri statusa:

### Disabled

Agent je isključen. Nijedna okidač se ne obrađuje i agent se ne pojavljuje u putanji dispečera. Njegova run history, analytics, i memory ostaju - ako ga ponovo omogućite kasnije, istorijski podaci su i dalje tu.

Use `Disabled` when:
- Želite izuzeti agenta iz rotacije bez gubitka.
- Agent se nepravilno ponaša i trebate ga odmah zaustaviti dok istražujete.
- Sezonski rotirate agente (npr. agent koji radi samo tokom praznika).

### Dry Run - default for new agents

Agent radi end-to-end - obrađuje okidače, poziva LLM, bira pozive alata, izračunava opravdanja i nivo povjerenja - ali **no real action is taken**. Svako pokretanje se beleži sa značkom **Dry Run** u [Istoriji pokretanja](#run-history).

Use `Dry Run` when:
- Novi agent je tek iz kutije. Svaki starter template po defaultu ide u Dry Run.
- Uredili ste prompt ili promijenili skup okidača i želite vidjeti kako promjena funkcioniše prije nego što je primijenite.
- Pokrećete a [test run / replay](#test-runs-replays) (replay-i prisiljavaju Dry Run bez obzira na status agenta).

Platforma naplaćuje tokene za Dry Run pokretanja - poziv LLM se i dalje dešava, preskaču se samo sporedni efekti. Budget caps se odnose i na Dry Run. Pogledajte [Pregled budžeta](#budgets-overview).

### Enabled

Agent preduzima stvarne akcije. Pozivi alata se izvršavaju - ili se stavljaju u red čekanja za [odobrenje](#approval-workflow) ako akcija zahtijeva odobrenje.

Use `Enabled` after dry-run output looks correct.

### Switching status

Možete flip-ovati između bilo koja dva statusa na edit formi. Switching from Dry Run to Enabled does not retroactively re-execute the dry-run actions - those stay as dry-run history. Novi okidači od tog trenutka nadalje run live.

Switching from Enabled to Disabled mid-run does **ne** abort an in-flight run. Trenutni okidač koji se izvršava završava (sa onim što je već započeo); sljedeći okidač se odbacuje jer je agent sada Disabled.

### Status during billing problems

Ako naplata vašeg tenanta postane nevažeća, svi agenti su efektivno pauzirani bez obzira na sačuvani status - okidači se odbacuju sa `BILLING_INVALID` dok se naplata ne obnovi. Sačuvano polje statusa se ne mijenja; dispatcher jednostavno odbija da pokrene. Pogledajte [Planovi i podobnost](#plans-and-eligibility).