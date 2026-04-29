Privzeto agent teče po celem vašem najemniku — na vsaki strani, v vsakem lokalnem jeziku. Razdelka **Scope** in **Locales** v obrazcu za urejanje vam dovolita, da to zožite.

### Omejitev na določene strani

Polje **Restrict to specific pages** sprejema en vzorec URL na vrstico, v sintaksi url-pattern glob. Agent se izvaja samo pri komentarjih, katerih URL strani se ujema z vsaj enim od vzorcev. Primeri:

- `/news/*` - katera koli stran pod `/news`.
- `/forums/*` - katera koli stran pod `/forums`.
- `/blog/2026/*` - katera koli stran pod `/blog/2026`.
- (več vrstic skupaj) - agent se zažene, če se **kateri koli** vzorec ujema.

Maksimum: 50 vzorcev na agenta. Vzorci morajo biti veljavni url-pattern globi — obrazec zavrne neveljavne z jasno napako.

Ko je polje **prazno**, se agent izvaja na vsaki strani v najemniku.

Ko je polje **neprazno**, agent deluje po načelu "fail closed": vsak sprožilec, katerega komentar nima `urlId` (npr. dogodki na ravni najemnika brez konteksta strani), je preskočen. To je namenoma — "omejeno na /news/*" se ne sme tiho razširiti na "vse".

### Omejitev na določene lokale

Dvojni izbor **Restrict to specific locales** sprejema FastComments ID-je lokalov (`en_us`, `zh_cn`, `de_de`, itd.). Agent se izvaja samo pri komentarjih, katerih zaznani lokal je na izbranem seznamu.

Zaznani lokal pride iz komentarjevega polja `locale`, ki ga nastavi vtičnik za komentarje ob objavi glede na lokal strani.

Ko **ni izbranih lokal**, se agent izvaja v vseh lokalih.

Ko je izbran **eden ali več lokalov**, agent deluje po načelu "fail closed": sprožilci brez komentarja ali sprožilci pri komentarjih brez polja `locale` se preskočijo.

### Združeno obsežno omejevanje

URL in lokalni filtri delujejo z logiko AND. Sprožilec sproži agenta le, če mu **oba** filtra to dovolita.

Uporabni vzorci:
- `/news/*` URL vzorec + `en_us` lokal - samo angleška novična sekcija.
- Brez URL filtra + več lokalov - po celotnem najemniku, a le za jezike, za katere je bil napisan prompt tega agenta.

### Zakaj omejiti agenta

- **Stroški.** Omejevanje zmanjša količino sprožilcev, ki jih mora agent obdelati, in tako zmanjša porabo tokenov.
- **Pravilnost.** Povzetek, prilagojen tehničnim člankom, lahko na straneh s produkti da slabe rezultate. Omejevanje je ostrejše orodje kot da v pozivu prosite, naj "preskoči netehnične strani" v angleščini.
- **Vedenje, specifično za lokal.** Pozdravni agent, ki piše samo v nemščini, bi se moral izvajati samo pri komentarjih z nemškim lokalom. Združite obseg `de_de` z nemško tonalnostjo v [initial prompt](#personality-prompt).

### Česa omejevanje ne počne

- Ne spremeni **števila slotov agentov** (glej [Plans and Eligibility](#plans-and-eligibility)) — omejeni agent še vedno zasede en slot.
- Ne spremeni [Budget caps](#budgets-overview) — dnevne in mesečne omejitve na agenta veljajo za vse ujemajoče se sprožilce.
- Ne omeji retrospektivno preteklih izvedb — zgodovina izvajanja prikazuje vse, kar je agent naredil, tudi če ga pozneje omejite strožje.