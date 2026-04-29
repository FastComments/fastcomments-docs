**Izboljšaj poziv** je potek dela za urejanje agentovega [začetnega poziva](#personality-prompt) v odziv na posebne odločitve, s katerimi se ne strinjate. Zažene se iz [predala odobritev](#approval-workflow).

### Kdaj ga uporabiti

Ko večkrat zapored zavračate enako vrsto odobritve — "agent hoče prepovedati ljudi zaradi uporabe močnega jezika brez cilja" — je agentov poziv vzvod za rešitev tega problema. Izboljšaj poziv je vodena metoda za:

1. Izbrati specifično odobritev, ki predstavlja slabo odločitev.
2. Urediti poziv s popolnim kontekstom tega, kar je agent storil in zakaj.
3. Shraniti nov poziv agentu.

Rezultat je agent, ki v prihodnje verjetno ne bo sprejel iste odločitve.

### Zagon poteka

Iz predala odobritev na `/auth/my-account/ai-agent-approvals`:

1. Odprite **zavrnjeno** odobritev. Pot je strogo omejena na vse razen REJECTED - pending in execution-failed odobritve niso upravičene.
2. Kliknite **Izboljšaj poziv**.

Pristanete v vmesniku za izpopolnjevanje poziva na `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Kaj prikazuje stran

- **Odobritev** - agentov `toolName` in `justification` za zavrnjeno odločitev (tukaj ni prikazan celoten LLM prepis).
- **Trenutni poziv** - agentov shranjeni [začetni poziv](#personality-prompt).
- **Vnos povratne informacije** - vnesete **povratne informacije**, v katerih opišete, kaj naj se spremeni (do 2000 znakov). LLM nato iz vaše povratne informacije ustvari predlagani nov poziv.
- **Združeni inline diff** - en sam inline diff med trenutnim in predlaganim pozivom (rdeče za odstranjeno, zeleno za dodano).

Kontekst odobritve ostane pripet na vrhu, tako da se lahko med urejanjem še naprej sklicujete na "zadevo, ki jo odpravljam".

### Shrani

Shranjevanje posodobi agentovo polje `initialPrompt`. Pretekli zagon(i) (in pretekle odobritve) se ne ponovijo retroaktivno — nov poziv vpliva le na prihodnje sprožitve. Če želite preveriti, ali nov poziv odpravi težavo, zaženite [preizkusno zaganjanje / replay](#test-runs-replays) za zadnjih 7 dni in preverite, ali bi novi poziv še vedno ustvaril zavrnjeno odobritev.

### Česa postopek ne počne

- Ne ureja **smernic skupnosti** — to polje ima svoj urejevalnik v glavnem obrazcu za urejanje agenta.
- Ne ureja **triggers**, **allowed tools** ali **approval gating** — ti ostanejo na glavnem obrazcu za urejanje.
- Ne verzionira poziva z možnostjo povrnitve. Prejšnji poziv ni shranjen v ločeni zbirki zgodovine. Če potrebujete vračilo, skopirajte trenutni poziv v svoj sistem za sledenje pred urejanjem.

### Zakaj združiti Izboljšaj poziv z replay

Urejanje poziva brez testiranja rezultata temelji na veri. Priporočeni cikel:

1. Zavrzi odobritev.
2. Izboljšaj poziv.
3. Zaženi [preizkusno zaganjanje](#test-runs-replays) za zadnjih 7 dni.
4. Poglejte zavihek **Deltas**. Ali je nov poziv premaknil slabo odločitev iz "bi naredil" v "ne bi naredil"? Ali je pomotoma premaknil tudi dobre odločitve ven?
5. Ponovite.

Tri ali štiri ponovitve cikla izboljšaj + replay običajno zadoščajo za stabilen poziv pri agentu za moderiranje.

### Neposredna alternativa urejanju

Ni nujno, da uporabljate Izboljšaj poziv — lahko tudi neposredno uredite agenta na glavnem obrazcu za urejanje. Edina prednost Izboljšaj poziv je, da pripne specifičen spodleteli primer, tako da ne izgubite slediti temu, za kaj popravljate.