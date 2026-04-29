Če kliknete **Ogled** v vrstici v [Zgodovini zagonov](#run-history), se odpre stran s podrobnostmi za posamezen zagon. Tu preberete razmišljanje agenta in ocenite njegove odločitve.

### Zgoraj: povzetek zagona

- **Agent** - kateri agent je izvedel zagon.
- **Kdaj** - časovni žig.
- **Status** - Začeto / Uspeh / Napaka, plus značko **Preizkusni zagon**, če velja.
- **Strošek** - strošek na zagon v valuti vašega najemnika.
- **Strošek na dejanje** - strošek deljen s številom dejanj, ki niso v čakalnem stanju; uporaben za odkrivanje nenavadno dragih zagonov.

### Izvedena dejanja

Seznam, v vrstnem redu, vseh klicev orodij, ki jih je zagon izvedel. Vsak vnos prikazuje:

- **Oznaka dejanja** - "Napisal je komentar", "Označil komentar kot spam", "Prepovedal uporabnika" itd. Oznaka je preslikana iz enum tipa dejanja.
- **Referenčni ID** - prizadet ID komentarja, uporabnika ali značke, prikazan v monospace pisavi (ni povezava).
- **Razlaga agenta** - utemeljitev, ki jo je agent priložil klicu.
- **Zaupanje** - agentova lastna ocena zaupanja, prikazana v odstotkih.
- **Značka čakajoče odobritve** - če je dejanje v vrsti v [predalu za odobritve](#approval-workflow) namesto izvedeno.

Če zagon ni izvedel nobenih dejanj, odsek prikazuje: "Med tem zagonom niso bila izvedena nobena dejanja."

### Prepis LLM

Pod dejanji je celoten prepis pogovora agenta z LLM:

- **Sistem** - sistemski prompt (sufiks platforme + vaš začetni poziv + smernice skupnosti).
- **Uporabnik** - kontekstno sporočilo, ki opisuje sprožilec.
- **Pomočnik** - odgovori modela, vključno s klici orodij.
- **Orodje** - rezultat orodja, posredovan nazaj modelu (npr. kaj je vrnil `search_memory`).

Dolga sporočila so zložljiva; kliknite Razširi / Strni, da prikažete.

### Branje prepisov

Prepis je najpomembnejša stran za prilagajanje. Ko agent sprejme odločitev, s katero se ne strinjate, ga preberite nazaj, da vidite:

- Kaj je model **videl** (kontekstno sporočilo uporabnika).
- Kaj se je model **odločil** (klici orodij Pomočnika).
- Kaj je model **upošteval** (rezultati orodij – npr. ali je agent zares poklical `search_memory` in ali je kaj našel, preden je izrekel prepoved).

Če model dosledno dela isto vrsto napake, uredite [začetni poziv](#personality-prompt) - ali uporabite [Izboljševanje pozivov](#refining-prompts) iz zavrnjene odobritve.

### Sklici dejanj

Referenčni ID-ji so prikazani v monospace pisavi (ne kot povezave):

- Komentarji: ID komentarja.
- Uporabniki: ID uporabnika.
- Značke: ID značke.

ID lahko kopirate, da poiščete prizadeti zapis na ustrezni strani za moderacijo/administracijo.

### Kaj manjka pri preizkusnem zagonu

Preizkusni zagon prikaže enaka dejanja, utemeljitve in ocene zaupanja. Edina razlika je značka **Preizkusni zagon** v vrstici stanja. Referenčni ID-ji za komentarje / uporabnike / značke so še vedno prikazani - agent jih preprosto ni vplival.

### Napake

Za zagone v stanju `Error` stran s podrobnostmi prikaže osnovno sporočilo o napaki. Pogoste napake:

- **Ni konfiguriranega LLM API ključa** - napačna konfiguracija najemnika ali platforme.
- **Potekel čas klica LLM** - ponudnik LLM je bil počasen ali nedosegljiv.
- **Napaka pri razporejanju orodja** - agent je izbral orodje z napačnimi argumenti (npr. ID komentarja, ki ne obstaja več).
- **Proračun je bil porabljen med zagonom** - agentu je bila dosežena omejitev med izvajanjem zagona. Zagon je bil ustavljen.

Napake ne razveljavijo delnih dejanj - vsi klici orodij, dokončani pred napako, ostanejo.