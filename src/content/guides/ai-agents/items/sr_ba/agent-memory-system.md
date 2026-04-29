Agent memory is a tenant-scoped, **shared** key-value pool that every agent in your tenant can read from and write to. It exists so agents can carry context across runs.

### Zašto memorija postoji

LLM context is per-run. Without memory, an agent that issues a warning to a user has no way to know about that warning the next time it sees the same user. The platform's escalation policy - "warn before banning" - depends on the agent being able to find the prior warning. Memory is what makes that work.

### Dva tipa memorije

- **WARNING** - written automatically as part of the [`warn_user`](#tool-warn-user) flow. The agent does not write `WARNING` records by hand; they are a side effect of warning a user.
- **NOTE** - written by [`save_memory`](#tools-overview). General-purpose context the agent wants future agents to know.

The escalation policy looks specifically for `WARNING` records when deciding whether a ban is justified.

### Tenant-scoped, agent-shared

All agents in your tenant share **one memory pool**. A note saved by Agent A is visible to Agent B's `search_memory` calls. This is intentional - you want a triage agent's notes to inform a moderator agent's decisions.

`tenantId` is set by the executor from the agent's own tenant - never from LLM args - so cross-tenant memory leaks are impossible by construction.

### Šta sadrži zapis u memoriji

Each memory entry contains:

- **Ko ga je napisao**, i kada.
- **O kome se radi** - korisnik kojeg ova memorija opisuje. Agent ne može to izmisliti; platforma to automatski popunjava iz onoga što je pokrenulo agenta.
- **Skriveni signal alternativnog naloga** - platforma također (privatno) bilježi IP otisak prsta komentara koji je pokrenuo događaj, tako da buduće pretrage memorije mogu istaknuti bilješke o drugim nalozima koji su objavljivali s iste IP adrese. Otisak nikada nije prikazan agentu niti LLM-u.
- **Sama bilješka** - do 2000 znakova slobodnog teksta.
- **Tagovi** za dohvat - do 10 kratkih tagova.
- **Vrsta** - ili upozorenje ili opća bilješka.
- **Neobavezna poveznica na komentar** - ako je memorija vezana za određeni komentar.

### Ponašanje pretrage

[`search_memory`](#tools-overview) vraća do 25 zapisa, sortirano od najnovijih, automatski ograničeno na (korisnika koji je pokrenuo) ILI (druge naloge sa iste IP adrese pokretača). Rezultati su također ograničeni na ukupno 8000 znakova kroz sav vraćeni sadržaj - stariji unosi se izbacuju ako se pređe limit.

Agent ne prosljeđuje `userId` ili `targetIpHash`. Oba postavlja izvršitelj.

### Trajnost

Memory has **no TTL**. Records persist until explicitly removed. WARNING records about a user are intentionally never auto-deleted - the escalation history must be findable indefinitely or the platform's "search before banning" check is meaningless.

Tri načina na koje se memorija uklanja:

- Moderator obriše osnovni komentar - sva memorija vezana za taj komentar se kaskadno uklanja.
- Korisnik se obriše - svi zapisi memorije o tom korisniku uklanjaju se u istoj transakciji.
- Vaš tenant se obriše.

Danas ne postoji admin UI za brisanje pojedinačnih zapisa memorije.

### Memorija u dry-run režimu

Agenti u dry-run režimu **ne** pišu memoriju. To je namjerno: hipotetičke odluke dry-run agenta ne bi trebale zagađivati zajednički spremnik memorije. Vraćanje čitanja putem `search_memory` radi normalno u dry-run režimu - agent može vidjeti stvarne memorije od live agenata - samo ne može ništa dodati.

### Memorija u replay-ima

Isto kao i dry-run: replay agenti ne pišu memoriju. Replay-ovi su samo za pregled. Vidi [Test Runs (Replays)](#test-runs-replays).

### Sažetak ograničenja

| Ograničenje | Vrijednost |
|---|---|
| Maksimalna dužina sadržaja memorije | 2000 znakova |
| Maksimalna dužina oznake memorije | 64 znakova |
| Maksimalan broj oznaka memorije | 10 |
| Maksimalna dužina upita u memoriji | 200 znakova |
| Ograničenje rezultata pretrage memorije | 25 zapisa |
| Ukupni limit sadržaja pretrage memorije | 8000 znakova |

### Pogledajte također

- [Tool: save_memory](#tools-overview) za pisanje.
- [Tool: search_memory](#tools-overview) za čitanje.
- [Tool: warn_user](#tool-warn-user) - jedini alat koji piše memoriju tipa WARNING.
- [Tool: ban_user](#tool-ban-user) - sistemski prompt zahtijeva da se prije ovoga pozove `search_memory`.

---