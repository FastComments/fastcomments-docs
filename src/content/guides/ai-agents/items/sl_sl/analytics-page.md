Analytics je nadzorna plošča čez agente. Dosegljiva je na strani AI agentov preko zavihka **Analytics** (na ravni najemnika) ali za posameznega agenta preko gumba **Analytics** v vrstici vsakega agenta.

### Filter

Spustni meni na vrhu - **Vsi agenti** ali določen agent. Preostali del strani se ustrezno prefiltrira.

### Uporaba proračuna

Štiri vrstice napredka, ki prikazujejo porabo v tekočem obdobju glede na omejitev:

- **Agent danes** (ko je filtrirano na določenega agenta) - dnevna omejitev agenta.
- **Agent ta mesec** - mesečna omejitev agenta.
- **Račun danes** - dnevna omejitev najemnika.
- **Račun ta mesec** - mesečna omejitev najemnika.

Ko omejitev ni nastavljena, vrstica prikazuje "(ni nastavljene omejitve)" in prikaže surovo porabo.

### Dnevni stroški (zadnjih 30 dni)

Tabela dnevnih stroškov v valuti vašega najemnika za izbran obseg. Uporabno za odkrivanje:

- **Nenadnih skokov stroškov** - običajno zaradi zanke brez izhoda ali viralnega komentarja, ki sproži množico dogodkov.
- **Odstopanja stroškov** - postopno naraščanje dnevnih stroškov, ko raste vaša skupnost.

### Izvedene akcije

Razčlenitev vrst akcij v tekočem mesecu - "Napisal komentar: 47", "Označil komentar kot neželen: 12" ipd. Uporabno za preverjanje, ali agent deluje po pričakovanjih.

### Preskočeni sprožilci (ta mesec)

Števila združena po [vzroku zavrnitve](#drop-reasons):

- Prekoračeno agentovo dnevno / agentovo mesečno / najemnika dnevno / najemnika mesečno.
- Omejeno zaradi omejitve hitrosti.
- Sočasnost je nasičena.

Če tukaj vidite zavrnitve, vaš agent dosega proračun ali omejitev hitrosti in zamudi sprožilce, ki bi jih sicer izvajal. Oglejte si [Vzroke zavrnitve](#drop-reasons).

### Poskusni zagon (dry-run) proti živemu (ta mesec)

- **Omogočeni teki** - število izvedb, ki so ta mesec izvedle dejanske akcije.
- **Poskusni teki** - število izvedb v načinu dry-run ta mesec.

Koristen signal za nastavljanje: povsem nov agent, ki še ni bil preklopljen na Omogočen, bo prikazoval samo poskusne teke. Agent, ki je Omogočen, a ima v tem razdelku vse ničle, je neaktiven - bodisi ni sprožen, bodisi je izključen iz obsega, bodisi njegovi sprožilci niso pravilno konfigurirani.

### Najdražji agenti po mesečnih stroških

Ko je filter **Vsi agenti**, stran prikaže agente razvrščene po stroških v tekočem mesecu. Zaznava najdražjega agenta je prvi korak pri optimizaciji stroškov - običajno je odgovor "zostrite njegove [možnosti konteksta](#context-options)" ali "znižajte njegovo [omejitev proračuna](#budgets-overview)".

### Agenti pri ali blizu svoje omejitve

Razčlenitev agentov, katerih poraba je v tekočem obdobju pri ali blizu njihovih agentnih omejitev:

- **blizu omejitve** - nad konfigurabilnim odstotkom omejitve.
- **preko omejitve** - dejansko omejen, z {count} `dropped` sprožilci v tem obdobju.

Kliknite na agenta v tej tabeli, da dvignete omejitev, zožite obseg ali ga začasno ustavite.

### Povzetek računa

Ko je filter **Vsi agenti**:

- **Sprožilci danes** - število.
- **Sprožilci ta mesec** - število.
- Za vsak: priponka `dropped`, ki prikazuje, koliko je bilo preskočenih.

### Valuta

Vse denarne vrednosti so prikazane v valuti vašega najemnika.

### Česa ta stran ne prikazuje

- Ne prikazuje **razčlenitev stroškov po akcijah** - te so na [Pogled podrobnosti zagona](#run-detail-view).
- Ne prikazuje **prepisov** ali **odgovorov LLM**.
- Ne omogoča izvajanja dejanj nad agenti - urejanje, začasno ustavljanje in brisanje se izvajajo na seznamu agentov / strani za urejanje.