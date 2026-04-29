Po defaultu, agent se pokreće na celom vašem tenantu - svaka stranica, svaki lokal. Sekcije **Scope** i **Locales** na formi za uređivanje omogućavaju da to suzite.

### Ograniči na određene stranice

Polje **Restrict to specific pages** prihvata po jedan URL obrazac po liniji, u url-pattern glob sintaksi. Agent se pokreće samo na komentarima čiji URL stranice odgovara bar jednom od šablona. Primeri:

- `/news/*` - bilo koja stranica pod `/news`.
- `/forums/*` - bilo koja stranica pod `/forums`.
- `/blog/2026/*` - bilo koja stranica pod `/blog/2026`.
- (više linija zajedno) - agent se pokreće ako odgovara **bilo koji** obrazac.

Maksimum: 50 obrazaca po agentu. Obrasci moraju biti validni url-pattern globovi - forma odbacuje neispravne sa specifičnom greškom.

Kada je polje **prazno**, agent se pokreće na svakoj stranici u tenantu.

Kada polje **nije prazno**, agent radi po principu zatvorenog neuspeha: svaki okidač čiji komentar nema `urlId` (npr. događaji na nivou tenanta bez konteksta stranice) se preskače. Ovo je namerno — "ogrančeno na /news/*" ne bi trebalo tiho da pređe na "sve".

### Ograniči na određene lokale

Dual-list picker **Restrict to specific locales** prihvata FastComments locale ID-e (`en_us`, `zh_cn`, `de_de`, etc.). Agent se pokreće samo na komentarima čija detektovana lokalizacija je na izabranom spisku.

Detektovana lokalizacija dolazi iz komentara polja `locale`, koje widget za komentare postavlja u trenutku objave na osnovu lokalizacije stranice.

Kada **nijedan lokal** nije izabran, agent se pokreće za sve lokale.

Kada je izabran **jedan ili više lokala**, agent radi po principu zatvorenog neuspeha: okidači bez komentara, ili okidači na komentarima bez polja `locale`, se preskaču.

### Kombinovano ograničavanje

URL i lokalni filteri se kombinuju logičkim AND. Okidač pokreće agenta samo ako **oba** filtera to dozvoljavaju.

Korisni obrasci:
- `/news/*` URL obrazac + `en_us` lokal - samo engleska sekcija vesti.
- Nema URL filtera + više lokala - obuhvata ceo tenant, ali samo za jezike za koje je ovaj agentov prompt napisan.

### Zašto ograničiti agenta

- **Trošak.** Ograničavanje smanjuje broj okidača koje agent mora da procesuira, i time smanjuje potrošnju tokena.
- **Ispravnost.** Prompt za sumiranje prilagođen tehničkim člancima može dati loš rezultat na stranicama proizvoda. Ograničavanje je precizniji alat nego tražiti od prompta da "preskoči netehničke stranice" na engleskom.
- **Ponašanje specifično za lokal.** Pozdravni agent koji piše samo na nemačkom treba da se pokreće samo na komentarima sa nemačkim lokalom. Kombinujte opseg `de_de` sa tonom na nemačkom u [početnom promptu](#personality-prompt).

### Šta ograničavanje *ne* radi

- Ne menja **agent slot count** (pogledajte [Planovi i podobnost](#plans-and-eligibility)) — ograničeni agent i dalje zauzima jedan slot.
- Ne menja [Granice budžeta](#budgets-overview) — dnevni i mesečni limiti po agentu važe preko svih odgovarajućih okidača.
- Ne primenjuje se retroaktivno na prethodna izvršavanja — istorija izvršavanja prikazuje sve što je agent uradio, čak i ako kasnije strože podesite opseg.