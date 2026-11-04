Prema zadanim postavkama, agent radi na cijelom vašem tenantu - svaka stranica, svaki lokal. Sekcije **Scope** i **Locales** na obrascu za uređivanje omogućuju vam da to suzite.

### Ograniči na određene stranice

Polje **Restrict to specific pages** prihvaća jedan URL obrazac po retku, u url-pattern glob sintaksi. Agent radi samo na komentarima čiji URL stranice odgovara barem jednom od obrazaca. Primjeri:

- `/news/*` - bilo koja stranica ispod `/news`.
- `/forums/*` - bilo koja stranica ispod `/forums`.
- `/blog/2026/*` - bilo koja stranica ispod `/blog/2026`.
- (više redaka zajedno) - agent radi ako odgovara **bilo koji** obrazac.

Maksimum: 50 obrazaca po agentu. Obrasci moraju biti valjani url-pattern globovi - obrazac odbacuje neispravne s konkretnom greškom.

Kada je polje **prazno**, agent radi na svakoj stranici u tenantu.

Kada polje **nije prazno**, agent faila closed: svaki okidač čiji komentar nema `urlId` (npr. događaji na razini tenanta bez konteksta stranice) se preskače. To je po dizajnu - "scoped to /news/*" ne bi trebalo tiho pasti na "sve".

### Ograniči na određene lokale

Dvostruki odabir lista **Restrict to specific locales** prihvaća FastComments ID-jeve lokala (`en_us`, `zh_cn`, `de_de` itd.). Agent radi samo na komentarima čiji otkriveni lokal je u odabranom popisu.

Otkriveni lokal dolazi iz polja `locale` komentara, koje widget za komentare postavlja u trenutku objave na temelju lokala stranice.

Kada **nema odabranih lokala**, agent radi na svakom lokalu.

Kada je odabran **jedan ili više lokala**, agent faila closed: okidači bez komentara, ili okidači na komentarima bez polja `locale`, se preskaču.

### Kombinirano opsegivanje

URL i lokalni filtri se kombiniraju s AND. Okidač pokreće agenta samo ako **oba** filtra dopuštaju.

Korisni obrasci:
- URL obrazac `/news/*` + lokal `en_us` - samo engleska sekcija vijesti.
- Bez URL filtra + više lokala - na razini cijelog tenanta, ali samo za jezike za koje je prompt ovog agenta napisan.

### Zašto opsegiti agenta

- **Trošak.** Opsegivanje smanjuje količinu okidača koje agent mora obraditi, a time i potrošnju tokena.
- **Točnost.** Prompt za sažimanje podešen za tehničke članke može proizvesti loš izlaz na stranicama proizvoda. Opsegivanje je oštriji alat od traženja promptu da "preskoči netehničke stranice" na engleskom.
- **Ponašanje specifično za lokal.** Pozdravni agent koji piše samo na njemačkom trebao bi raditi samo na komentarima njemačkog lokala. Kombinirajte opseg lokala `de_de` s njemačkim tonom u [početnom promptu](#personality-prompt).

### Što opsegivanje *ne* radi

- Ne mijenja **broj slotova za agente** (vidi [Plans and Eligibility](#plans-and-eligibility)) - agent s opsegom i dalje zauzima jedan slot.
- Ne mijenja [Budget caps](#budgets-overview) - dnevni i mjesečni limiti po agentu primjenjuju se na sve odgovarajuće okidače.
- Ne opsežuje retroaktivno prošle pokretanja - povijest pokretanja prikazuje sve što je agent radio, čak i ako ga kasnije suzite.