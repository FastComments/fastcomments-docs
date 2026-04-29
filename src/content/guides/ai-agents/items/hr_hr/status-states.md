Agent ima jedan od tri statusa:

### Disabled

Agent je isključen. Nijedni okidači se ne obrađuju i agent se ne pojavljuje u dispatch putanji. Njegova povijest pokretanja, analitika i memorija ostaju — ako ga ponovo omogućite kasnije, povijesni podaci su i dalje dostupni.

Use `Disabled` when:
- Želite izvaditi agenta iz rotacije bez gubitka.
- Agent se nepravilno ponaša i trebate ga odmah zaustaviti dok istražujete.
- Sezonski rotirate agente (npr. pozdravljač koji radi samo za praznike).

### Dry Run - zadano za nove agente

Agent radi od početka do kraja - obrađuje okidače, poziva LLM, odabire pozive alata, izračunava opravdanja i razinu povjerenja - ali **ne poduzimaju se stvarne radnje**. Svako pokretanje se bilježi s oznakom **Dry Run** u [Povijest izvođenja](#run-history).

Use `Dry Run` when:
- Novi agent je tek iz kutije. Svaki početni predložak započinje u dry-runu.
- Uredili ste prompt ili promijenili skup okidača i želite vidjeti kako se promjena ponaša prije nego što je primijenite.
- Pokrećete [testno izvođenje / reprizu](#test-runs-replays) (reprize prisiljavaju dry-run bez obzira na status agenta).

Platforma naplaćuje tokene za dry-run pokretanja - poziv LLM-a i dalje se događa, samo se nuspojave preskaču. Granice proračuna također se primjenjuju na dry-run. Pogledajte [Pregled proračuna](#budgets-overview).

### Enabled

Agent poduzima stvarne radnje. Pozivi alata se izvršavaju — ili se stavljaju u red čekanja za [odobrenje](#approval-workflow) ako je radnja ograničena.

Use `Enabled` after dry-run output looks correct.

### Prebacivanje statusa

Možete se prebacivati između bilo koja dva statusa u obrascu za uređivanje. Prebacivanje iz Dry Run u Enabled ne ponovo izvršava retroaktivno akcije dry-runa — one ostaju kao povijest dry-runa. Novi okidači od tog trenutka nadalje rade uživo.

Prebacivanje iz Enabled u Disabled usred pokretanja **ne** prekida pokretanje koje je u tijeku. Trenutno izvršavajući okidač dovršava se (s onim što je već započeo); sljedeći okidač se odbacuje jer je agent sada Disabled.

### Status tijekom problema s naplatom

Ako naplata vašeg tenant-a postane nevažeća, svi agenti su zapravo pauzirani bez obzira na spremljeni status - okidači se odbacuju s `BILLING_INVALID` dok se naplata ne obnovi. Polje spremljenog statusa se ne mijenja; dispatcher jednostavno odbija pokretanje. Pogledajte [Planovi i podobnost](#plans-and-eligibility).