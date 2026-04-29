Na stranici [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) možete stvoriti agenta na dva načina:

- **Iz predloška.** Kliknite **Browse templates** i odaberite jednog od četiri ugrađena početna agenta. Obrazac dolazi unaprijed ispunjen i status agenta je **Probni način**. Pogledajte [Starter Templates](#starter-templates).
- **Od nule.** Kliknite **Create new agent**. Obrazac dolazi prazan.

Bilo koji način, isti obrazac je ono što ćete kasnije spremati i uređivati. Ova stranica vodi kroz obrazac od vrha do dna.

### Osnovno

- **Internal name.** Kratki identifikator koji se koristi samo u administratorskim sučeljima (povijest pokretanja, analitika, zapisnici revizije). Mala slova s donjim crticama dobro funkcioniraju: `moderator`, `welcome_greeter`. Ako je interno ime predloška već zauzeto, obrazac automatski dodaje sufiks (`tos_enforcer_2`, itd.).
- **Display name.** Prikazuje se javno kad agent objavi komentar. Ovo vide vaši čitatelji.
- **Status.** Onemogućeno, Probni način ili Omogućeno. Novi agenti uvijek su zadano u Probnom načinu. Pogledajte [Status States](#status-states).

### Model

Odaberite LLM. Pogledajte [Choosing a Model](#choosing-a-model).

### Proračun

Opcionalni dnevni i mjesečni limiti u valuti vašeg računa, plus kontrolna lista **Alert thresholds** (zadano 80% i 100%). Pogledajte [Budgets Overview](#budgets-overview) i [Budget Alerts](#budget-alerts).

### Osobnost

Početni prompt je sistemski prompt koji definira ton, ulogu i pravila odlučivanja. Obični tekst, bez sintakse predložaka. Pogledajte [Personality and the Initial Prompt](#personality-prompt).

### Kontekst

Skup polja Kontekst ima tri potvrdna okvira, područje za smjernice i ulaze za opseg:

- Uključi roditeljski komentar i prethodne odgovore u istom threadu.
- Uključi korisnikov trust factor, starost računa, povijest zabrana i nedavne komentare.
- Uključi naslov stranice, podnaslov, opis i meta tagove.
- Neobavezni blok teksta **Community guidelines** koji se dodaje ispred svakog prompta.
- **Restrict to specific pages** - dopuštena lista URL obrazaca (po jedan po retku). Prazno znači za cijeli tenant.
- **Restrict to specific locales** - dopuštena lista lokaliteta putem alata za odabir s dvostrukom listom. Prazno znači za sve lokalitete.

Više konteksta daje bolje odluke, ali povećava trošak tokena po pokretanju. Pogledajte [Context Options](#context-options), [Community Guidelines](#community-guidelines) i [Scope: URL and Locale Filters](#scope-url-locale).

### Okidači

Odaberite barem jedan događaj s popisa. Za okidače vote-threshold i flag-threshold također morate postaviti prag. Neobavezno polje **Delay before running** odgađa izvršenje nakon što se okidač aktivira (korisno za flag-threshold gdje se glasovi još obračunavaju). Pogledajte [Trigger Events Overview](#triggers-overview) i [Deferred Triggers](#trigger-deferred-delay).

### Dozvoljeni pozivi alata

Označite **Allow any tool calls** da biste izložili puni set alata. U suprotnom označite specifične alate koje agent smije koristiti - nedozvoljeni alati se uklanjaju iz palete modela i odbijaju pri slanju. Pododjeljak **Ban options** stavlja destruktivne varijante zabrane (delete-all-comments, ban-by-IP) iza eksplicitnih pristanka. Pogledajte [Allowed Tool Calls Overview](#tools-overview) i [Tool: ban_user](#tool-ban-user).

### Odobrenja

Označite akcije koje moraju odobriti ljudi prije nego agent izvrši. Odobrenja se primjenjuju samo na alate koje agent smije pozvati; nedozvoljeni alati se jednostavno odbijaju. U EU regiji, ban_user je obavezno uključen prema članku 17. Uredbe o digitalnim uslugama. Pogledajte [Approval Workflow](#approval-workflow) i [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Obavijesti o odobrenjima

Ako su odobrenja omogućena, odaberite tko prima e-poruke:

- **Svi administratori i moderatori** - vlasnici računa, super administratori i administratori moderiranja komentara.
- **Određeni korisnici** - odabrani putem alata za odabir s dvostrukom listom.

Frekvenciju dostave za svakog recenzenta (odmah, satni sažetak, dnevni sažetak) postavlja se u njihovom profilu. Pogledajte [Approval Notifications](#approval-notifications).

### Statistika

Samo za čitanje. Ukupan broj pokretanja, vrijeme posljednjeg pokretanja i ID najnovijeg komentara kojeg je agent napisao (ako postoji).

### Spremi

Kliknite **Save agent**. Stranica se preusmjerava natrag na popis agenata. Novi agenti su odmah podobni za primanje okidača u probnom načinu.

### Uređivanje kasnije

Svaki redak na stranici s popisom agenata izlaže akcije po agentu: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals** i **Delete**. Uređivanje agenta ne utječe retroaktivno na već zabilježena pokretanja - povijest se čuva. Replay snapshotovi također zamrzavaju konfiguraciju agenta u trenutku kada je replay započet, tako da rezultati spremljenog replaya ostaju reproducibilni čak i nakon što uredite prompt.