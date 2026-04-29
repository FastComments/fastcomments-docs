The Warn tool šalje privatnu DM opomenu korisniku u vezi određenog komentara, a istovremeno bilježi opomenu u zajedničkoj [agent memory](#agent-memory-system). Ta dva zapisa su atomska - korisnik nikada ne vidi opomenu koja nije također zabilježena.

### Zašto postoji

Politika eskalacije platforme je **prvo opomeni, zabrani samo ako korisnik ponovo prekrši pravila**. The Warn tool je ono što tu politiku čini provedivom: daje korisniku priliku da se ispravi, a zapis o opomeni je ono što budući agent pronađe kada pretražuje memoriju prije nego što razmotri zabranu.

### Što ide u opomenu

Kratka poruka (ograničena na 1000 znakova) koja se korisniku prikazuje kao DM. Snažne opomene su:

- **Specific** - "Personal attacks on named users are not allowed in this community" bolje je od "your comment was flagged."
- **Short** - najviše nekoliko rečenica.
- **Actionable** - recite korisniku što treba promijeniti. "Please edit your comment to remove the named user, or it will be removed."

Poruku sami ne pišete; agent to radi, na temelju [initial prompt](#personality-prompt) i [community guidelines](#community-guidelines). Vaš zadatak je napisati prompt koji proizvodi dobre opomene.

### Kada ga dozvoliti

Za bilo kojeg agenta u stilu moderiranja. Predložak Moderator omogućuje ga prema zadanim postavkama.

### Odobrenja

Rjeđe se stavlja iza odobrenja nego [Ban user](#tool-ban-user). Vrijedi postaviti odobrenje tijekom prvih tjedana rada agenta kako biste mogli uočiti loše opomene prije nego što se pošalju, ali većina operatera ukloni odobrenje kad agent počne davati pouzdane rezultate.

### Vidi također

- [Ban user](#tool-ban-user) - sljedeći korak u eskalaciji.
- [Agent Memory System](#agent-memory-system) - gdje se pohranjuju zapisi o opomenama.

---