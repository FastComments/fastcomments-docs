Zgodovina izvedb (Run History) je dnevnik po agentu vsakega sprožilca, ki je tekel. Dosegljiva je s strani strani seznama agentov preko gumba **Runs**, ali neposredno na `/auth/my-account/ai-agents/{agentId}/runs`.

### What's on the page

Stran z paginirano tabelo z eno vrstico na izvedbo:

| Column | Meaning |
|---|---|
| Date | Kdaj se je sprožilec sprožil (ali kdaj je tekel odložen sprožilec). |
| Status | **Started**, **Success**, or **Error**. Ob tem se prikaže tudi značka **Dry Run**, če je bila izvedba v načinu dry-run. |
| Cost | Strošek na izvedbo v valuti vaše najemniške enote. Prazno za izvedbe v teku (Started). |
| Actions | Število klicev orodij v izvedbi. |
| Details | Gumb **View**, ki odpre [Run Detail View](#run-detail-view). |

### Status meanings

- **Started** - izvedba je v teku ali je obnemela pred dokončanjem. Izvedba, ki je nenavadno dolgo v stanju "Started", navadno pomeni potekel čas pri klicu LLM.
- **Error** - izvedba je končana, vendar je nekje spodletela - klic LLM je vrnil napako, odpovedalo je posredovanje orodja itd. Pogled podrobnosti vsebuje specifično napako.
- **Success** - izvedba je končana brez napake. Agent je lahko izvedel nič, eno ali več dejanj.

### Empty state

Ko agent nima izvedb, stran prikaže: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Ta zadnji del je nameren — [test run flow](#test-runs-replays) je priporočeni način za napolnitev Run History pri svežem agentu.

### What's not on the run history page

- **Live triggers that never dispatched** - sprožilec, ki je izpadel zaradi proračuna, obsega ali omejitve hitrosti, se ne prikaže na tej strani. Ti se pojavijo na [Analytics page](#analytics-page) pod "Triggers skipped".
- **Approvals** - čakajoča odobritev za dejanja, izvedena v tej izvedbi, živi v [approvals inbox](#approval-workflow). Dejanje se v pogledu podrobnosti izvedbe prikaže kot **Pending approval**.

### Retention

Posamezni zapisi izvedb se hranijo 90 dni, po tem času izvedba izginja iz zgodovine. Stroški in število sprožilcev se še naprej seštevajo v dolgoročnih povzetkih analitike, zato [Analytics page](#analytics-page) še vedno prikazuje zgodovinske vsote izven tega obdobja.

### Replays

Izvedbe, ustvarjene z replays, so privzeto izključene iz pogleda živih izvedb. Stran [Test Runs (Replays)](#test-runs-replays) je kraj, kjer si jih lahko ogledate.

### Filtering across agents

Tabela izvedb je po agentu. Ni pogleda izvedb čez več agentov - [Analytics page](#analytics-page) je povzetek čez agente. Če morate pregledati izvedbe čez več agentov, so dogodki Webhooks `trigger.succeeded` in `trigger.failed` tisti, ki jih lahko posredujete v vaš sistem.