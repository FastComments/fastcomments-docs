Sa stranice [AI agenata](https://fastcomments.com/auth/my-account/ai-agents) možete kreirati agenta na dva načina:

- **Iz šablona.** Kliknite **Browse templates** i izaberite jednog od četiri ugrađena početna agenta. Formular se popunjava unapred i status agenta je **Probni režim**. Pogledajte [Početni šabloni](#starter-templates).
- **Od nule.** Kliknite **Create new agent**. Formular je prazan.

Bilo koji od ova dva načina, isti formular je ono što ćete naknadno čuvati i uređivati. Ova stranica vodi kroz formular od vrha do dna.

### Osnovno

- **Internal name.** Kratki identifikator koji se koristi samo u administratorskim kontrolnim tablama (istorija pokretanja, analitika, revizorski zapisi). Mala slova sa donjim crticama dobro funkcionišu: `moderator`, `welcome_greeter`. Ako je interni naziv iz šablona već zauzet, formular automatski dodaje sufiks (`tos_enforcer_2`, itd.).
- **Display name.** Prikazuje se javno kad god agent objavi komentar. Ovo je ono što vaši čitaoci vide.
- **Status.** Onemogućeno, Probni režim, ili Omogućeno. Novi agenti su po defaultu u Probnom režimu. Pogledajte [Stanja statusa](#status-states).

### Model

Izaberite LLM. Pogledajte [Izbor modela](#choosing-a-model).

### Budžet

Opcionalni dnevni i mesečni limiti u valuti vašeg naloga, plus lista za potvrdu **Alert thresholds** (podrazumevano 80% i 100%). Pogledajte [Pregled budžeta](#budgets-overview) i [Upozorenja budžeta](#budget-alerts).

### Ličnost

**Initial prompt** je sistemski prompt koji definiše ton, ulogu i pravila odlučivanja. Običan tekst, bez sintakse šablona. Pogledajte [Ličnost i početni prompt](#personality-prompt).

### Kontekst

Kontekst polje sadrži tri potvrđivačka polja, tekstualni prostor za smernice i ulaze opsega:

- Uključi roditeljski komentar i prethodne odgovore u istom thread-u.
- Uključi faktor poverenja komentatora, starost naloga, istoriju zabrana i nedavne komentare.
- Uključi naslov stranice, podnaslov, opis i meta tagove.
- Opcionalni tekstualni blok **Smernice zajednice** koji se dodaje na početak svakog prompta.
- **Ograniči na određene stranice** - lista dozvoljenih URL obrazaca (po jedan po liniji). Prazno znači važi za ceo nalog.
- **Ograniči na određene lokale** - lista dozvoljenih lokaliteta putem selektora sa dve liste. Prazno znači svi lokaleti.

Više konteksta daje bolje odluke, ali povećava trošak tokena po pokretanju. Pogledajte [Opcije konteksta](#context-options), [Smernice zajednice](#community-guidelines) i [Opseg: URL i filtre lokaliteta](#scope-url-locale).

### Okidači

Izaberite najmanje jedan događaj sa liste. Za okidače tipa vote-threshold i flag-threshold morate takođe podesiti prag. Opcionalno polje **Delay before running** odlaže izvršenje nakon što okidač bude aktiviran (korisno za pragove flag-ovanja gde se glasovi još uvek konsoliduju). Pogledajte [Pregled događaja okidača](#triggers-overview) i [Odloženi okidači](#trigger-deferred-delay).

### Dozvoljeni pozivi alata

Označite **Allow any tool calls** da biste prikazali ceo paletu alata. U suprotnom označite konkretne alate koje agent sme da koristi - nedozvoljeni alati se uklanjaju iz palete modela i odbijaju pri dodeli. Pododeljak **Ban options** ograničava destruktivne varijante zabrana (delete-all-comments, ban-by-IP) iza eksplicitnih potvrda. Pogledajte [Pregled dozvoljenih poziva alata](#tools-overview) i [Alat: ban_user](#tool-ban-user).

### Odobrenja

Označite akcije koje moraju biti odobrene od strane čoveka pre nego što agent izvrši iste. Odobrenja se primenjuju samo na alate koje agent sme da pozove; nedozvoljeni alati se odmah odbijaju. U EU regionu, ban_user je zaključan po članu 17 Zakona o digitalnim uslugama. Pogledajte [Tok odobravanja](#approval-workflow) i [EU DSA: usklađenost sa članom 17](#eu-dsa-compliance).

### Obaveštenja o odobrenjima

Ako su odobrenja omogućena, izaberite kome se šalju mejlovi:

- **Svi administratori i moderatori** - vlasnici naloga, super admini i administratori moderacije komentara.
- **Određeni korisnici** - ručno izabrani putem selektora sa dve liste.

Frekvencija isporuke pojedinačnog recenzenta (odmah, satni sažetak, dnevni sažetak) podešava se u njihovom profilu. Pogledajte [Obaveštenja o odobrenjima](#approval-notifications).

### Statistika

Samo za čitanje. Ukupan broj pokretanja, vremenska oznaka poslednjeg pokretanja i ID najnovijeg komentara koji je agent napisao (ako postoji).

### Sačuvaj

Kliknite **Save agent**. Stranica će preusmeriti nazad na listu agenata. Novi agenti su odmah podobni da primaju okidače u probnom režimu.

### Kasnije uređivanje

Svaki red na stranici liste agenata prikazuje po-agent akcije: **Izmeni**, **Kloniraj**, **Izvršavanja**, **Reprodukcije**, **Probno pokretanje**, **Analitika**, **Odobrenja**, i **Obriši**. Uređivanje agenta ne menja unazad već zabeležena pokretanja - istorija se čuva. Snapshot-ovi reprodukcije takođe zamrzavaju konfiguraciju agenta u trenutku kada je reprodukcija pokrenuta, tako da rezultati sačuvane reprodukcije ostaju reproduktivni čak i nakon što izmenite prompt.