Orodje Warn pošlje uporabniku zasebno DM-opozorilo glede določenega komentarja, hkrati pa zabeleži opozorilo v deljenem [Sistemu spomina agenta](#agent-memory-system). Ti dve operaciji zapisovanja sta atomski - uporabnik nikoli ne vidi opozorila, ki ni tudi zabeleženo.

### Zakaj obstaja

Politika eskalacije platforme je **najprej opozori, prepovej šele, če uporabnik ponovno prestopi mejo**. Orodje Warn omogoča izvajanje te politike: daje uporabniku priložnost, da popravi vedenje, zapis opozorila pa najde bodoči agent, ko pred razmislekom o prepovedi pregleda spomin.

Orodje prav tako prepreči podvajanje: če je agent že izdal opozorilo istemu uporabniku glede istega komentarja, je drugo opozorilo neoperativno. Tako LLM, ki se zatakne ali znova sproži na istem komentarju, ne more uporabnika spamati z več opozorili.

### Kaj naj bo v opozorilu

Kratko sporočilo (omejeno na 1000 znakov), prikazano uporabniku kot DM. Močna opozorila so:

- **Specifično** - "Osebni napadi na poimenovane uporabnike niso dovoljeni v tej skupnosti" je boljše kot "vaš komentar je bil označen."
- **Kratko** - največ nekaj stavkov.
- **Izvedljivo** - povejte uporabniku, kaj naj spremeni. "Prosimo, uredite svoj komentar in odstranite poimenovanega uporabnika, sicer bo odstranjen."

Sporočila ne pišete sami; to stori agent na podlagi [začetnega poziva](#personality-prompt) in [smernic skupnosti](#community-guidelines). Vaša naloga je napisati poziv, ki ustvarja dobra opozorila.

### Kdaj dovoliti

Za katerega koli agenta za moderiranje. Predloga Moderator to privzeto omogoča.

### Odobritve

Manj pogosto omejeno kot [Blokiraj uporabnika](#tool-ban-user). Smiselno je omejiti uporabo v prvih tednih delovanja agenta, da lahko opazite slaba opozorila, preden se pošljejo, vendar večina upravljavcev odstrani omejitev, ko agent začne proizvajati zanesljive rezultate.

### Oglejte si tudi

- [Blokiraj uporabnika](#tool-ban-user) - naslednji korak pri eskalaciji.
- [Sistem spomina agenta](#agent-memory-system) - kjer so shranjeni zapisi o opozorilih.