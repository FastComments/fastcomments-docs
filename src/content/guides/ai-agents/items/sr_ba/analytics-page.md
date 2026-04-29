Analitika je kontrolna tabla za više agenata. Pristup je moguć sa stranice AI agenata putem kartice **Analitika** (za cijeli tenant) ili po agentu putem dugmeta **Analitika** u svakom redu agenta.

### Filter

Padajući meni na vrhu - **Svi agenti** ili određeni agent. Ostatak stranice se u skladu s tim preusmjerava.

### Korištenje budžeta

Četiri trake napretka koje pokazuju trošak u tekućem periodu u odnosu na ograničenje:

- **Agent danas** (kad je filter postavljen na određenog agenta) - dnevni limit po agentu.
- **Agent ovog mjeseca** - mjesečni limit po agentu.
- **Nalog danas** - dnevni limit tenanta.
- **Nalog ovog mjeseca** - mjesečni limit tenanta.

Kada ograničenje nije postavljeno, traka prikazuje "(no cap set)" i prikazuje sirovi trošak.

### Dnevni trošak (posljednjih 30 dana)

Tabela dnevnih troškova u valuti vašeg tenanta za odabrani opseg. Korisno za uočavanje:

- **Naglih skokova troškova** - obično nastaju zbog petlje koja je izmakla kontroli ili viralnog komentara koji širi okidače.
- **Odstupanja u troškovima** - postepeno povećanje dnevnih troškova kako vaša zajednica raste.

### Preduzete radnje

Raspodjela tipova radnji tokom tekućeg mjeseca - "Napisao komentar: 47", "Označio komentar kao spam: 12" i tako dalje. Korisno za provjeru da li agent radi ono što ste očekivali.

### Preskočeni okidači (ovaj mjesec)

Brojčane vrijednosti grupisane po [drop reason](#drop-reasons):

- Preko dnevnog / mjesečnog limita agenta ili dnevnog / mjesečnog limita naloga.
- Ograničeno brzinom (rate-limited).
- Konkurentnost zasićena.

Ako ovdje vidite padove, vaš agent dostiže budžet ili ograničenje brzine i propušta okidače na kojima bi inače izvršavao radnje. Pogledajte [Drop Reasons](#drop-reasons).

### Dry-run vs live (ovaj mjesec)

- **Enabled runs** - broj izvršenja koja su ovog mjeseca preduzela stvarne radnje.
- **Dry runs** - broj izvršenja u dry-run režimu ovog mjeseca.

Korisni signal za podešavanje: potpuno novi agent koji još nije promovisan u Omogućeno će pokazivati samo dry run-ove. Agent u stanju Omogućeno sa svim-nula vrijednostima u ovom odjeljku je neaktivan - ili nije pokrenut, ili je izvan opsega, ili njegovi okidači nisu ispravno konfigurirani.

### Najbolji agenti po mjesečnom trošku

Kada je filter **Svi agenti**, stranica prikazuje agente rangirane po trošku od početka mjeseca. Uočavanje najskupljeg agenta je prvi korak u optimizaciji troškova - obično rješenje je "suziti njegove [opcije konteksta](#context-options)" ili "smanjiti njegovo [ograničenje budžeta](#budgets-overview)".

### Agenti koji su na ili blizu limita

Rasčlanjenje po agentu za agente čiji je trošak na ili blizu njihovih limita po agentu u tekućem periodu:

- **near cap** - iznad konfigurabilnog procenta limita.
- **over cap** - zaista ograničen, sa `{count} dropped` okidača u tom periodu.

Kliknite na agenta iz ove tabele da podignete limit, suzite opseg ili ga pauzirate.

### Sažetak naloga

Kada je filter **Svi agenti**:

- **Triggers today** - broj.
- **Triggers this month** - broj.
- Za svako: sufiks `dropped` koji pokazuje koliko je preskočeno.

### Valuta

Sve novčane vrijednosti prikazane su u valuti vašeg tenanta.

### Šta ova stranica ne radi

- Ne prikazuje **raspodjelu troškova po radnji** - to je na [Run Detail View](#run-detail-view).
- Ne prikazuje **transkripte** ili **LLM odgovore**.
- Ne omogućava radnje nad agentima - uređivanje, pauziranje, brisanje se rade iz liste agenata / stranice za uređivanje.