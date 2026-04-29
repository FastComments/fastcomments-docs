The **Community guidelines** polje na obrazcu za urejanje je neobvezni blok politik, vključen v sporočilo v kontekstu vloge uporabnika pri vsakem zagonu tega agenta. Je ograjeno kot nezaupljivo besedilo (isto ograjanje, ki ga platforma uporablja za vsebine komentarjev in druge vsebine, ki jih vnesejo uporabniki), zato ga model obravnava kot referenco politike, ne kot sistemska navodila. To je kanonično mesto za zapisovanje "katere vedenje je na tej strani dovoljeno in katerega ne", tako da ga agent dosledno uporablja.

### Kako se razlikuje od začetnega poziva

- **Začetni poziv** - vloga agenta in stil odločanja. "Vi ste moderator. Raje opozorite kot prepovedujte."
- **Smernice skupnosti** - pravila vaše skupnosti, v jezikovni obliki politike. "Brez osebnih napadov. Brez promocijskih povezav z računov, starejših manj kot 24 ur. Off-topic komentarji se lahko odstranijo, če je nit razgreta."

Oba toketa vstopata v isto okno konteksta, vendar na različnih plasteh - začetni poziv je del sistemske vloge, dokument smernic pa je ograjeno besedilo znotraj sporočila v kontekstu vloge uporabnika. Ta delitev olajša urejanje, če želite posodobiti eno brez ponovnega branja drugega.

### Kaj je dober dokument smernic

Kratek, konkreten dokument, napisan s strani človeka. Seznami delujejo bolje kot proza:

[inline-code-attrs-start title = 'Primer smernic skupnosti'; type='text' inline-code-attrs-end]
[inline-code-start]
Dovoljeno:
- Vsebinsko nestrinjanje, tudi močno izraženo.
- Povezave do izvornih virov, tudi z novimi računi.
- Pripombe izven teme, če matična nit to dopušča.

Prepovedano:
- Osebni napadi proti določenim navedenim uporabnikom.
- Doxxing ali deljenje zasebnih informacij.
- Koordinirana promocijska dejavnost (več komentarjev, ki pritiskajo isto zunanjo povezavo).
- Komentarji, ki obstajajo le zato, da odvrnejo razpravo.

Mejno:
- Močan jezik brez tarče. Dovoljeno, če ni usmerjeno proti osebi.
- Politične teme izven teme strani. Izven teme; najprej opozorite, ne odstranjujte, razen če so vztrajne.
[inline-code-end]

Agent to uporablja pri vsakem zagonu. Če spremenite smernice, sprememba začne veljati ob naslednjem sprožilcu - preteklih zaganjanj ne ocenjuje nazaj.

### Česa sem ne vnašati

- **Navodila za oblikovanje izhoda** ("odgovori v HTML", "uporabi emodžije"). Ta spadajo v [začetni poziv](#personality-prompt).
- **Lokalizirano besedilo.** Dokument smernic, tako kot poziv, je **samo v angleščini** iz istega razloga - strojno prevajanje lahko tiho spremeni vedenje agenta. Če imate politike, ki se razlikujejo po lokaciji, jih vse napišite v angleščini v tem enem dokumentu in strukturirajte dokument kot "za strani v nemščini: ..."
- **Dolge citate zunanjih politik.** Para-frazirajte. Dolg kontekst povečuje stroške žetonov pri vsakem zagonu.
- **Osebni podatki ali skrivnosti.** To besedilo se pošlje ponudniku LLM pri vsakem zagonu.

### Dolžina

Polje je omejeno na **4000 znakov** (to uveljavlja tako obrazec kot pot za shranjevanje). Stroški žetonov pri vsakem zagonu so sorazmerni z dolžino, zato je običajno nekaj sto besed običajno dovolj. Če vaše skupnostne politike obsegajo več strani, povzemi dele, ki jih agent potrebuje, v navodilo posebej za to polje.

### Verzije

Za dokument smernic ni vgrajene zgodovine različic - agent uporablja najnovejšo shranjeno vrednost. Če želite zgodovino, kopirajte dokument v svoj sistem za sledenje pred vsako večjo spremembo. Tok [Izboljševanje pozivov](#refining-prompts) lahko zabeleži spremembe *začetnega poziva*, vendar ne vodi verzij dokumenta smernic.