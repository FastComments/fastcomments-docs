Po zadanim postavkama, agent radi preko cijelog vašeg tenanta - svake stranice, svakog lokaliteta. Sekcije **Scope** i **Locales** na obrascu za uređivanje omogućuju da to suzite.

### Restrict to specific pages

Polje **Restrict to specific pages** prihvaća jedan URL obrazac po retku, u url-pattern glob sintaksi. Agent se pokreće samo na komentarima čiji URL stranice odgovara barem jednom od obrazaca. Primjeri:

- `/news/*` - bilo koja stranica pod `/news`.
- `/forums/*` - bilo koja stranica pod `/forums`.
- `/blog/2026/*` - bilo koja stranica pod `/blog/2026`.
- (više linija zajedno) - agent se pokreće ako se **bilo koji** obrazac poklapa.

Maksimum: 50 obrazaca po agentu. Obrasci moraju biti valjani url-pattern globovi - oblik odbacuje nepravilne s određenom greškom.

Kad je polje **prazno**, agent se pokreće na svakoj stranici u tenant-u.

Kad je polje **ne-prazno**, agent radi po principu 'fail closed': svaki trigger čiji komentar nema `urlId` (npr. događaji na razini tenanta bez konteksta stranice) se preskače. To je namjerno - "scope na /news/*" ne bi smjelo tiho pasti na "sve".

### Restrict to specific locales

Dual-list picker **Restrict to specific locales** prihvaća FastComments locale ID-jeve (`en_us`, `zh_cn`, `de_de`, itd.). Agent se pokreće samo na komentarima čiji je otkriveni lokalitet u odabranom popisu.

Otkriveni lokalitet dolazi iz polja `locale` komentara, koje postavlja widget za komentare prilikom objave na temelju lokaliteta stranice.

Kada **nisu odabrani lokaliteti**, agent se pokreće na svim lokalitetima.

Kada je **odabran jedan ili više lokaliteta**, agent radi po principu 'fail closed': triggeri bez komentara, ili triggeri na komentarima koji nemaju polje `locale`, se preskaču.

### Combined scoping

URL i filteri lokaliteta rade s logikom AND. Trigger pokreće agenta samo ako **oba** filtera to dopuštaju.

Koristan primjeri:
- `/news/*` URL obrazac + `en_us` lokalitet - samo engleski news odjeljak.
- Nema URL filtera + više lokaliteta - za cijeli tenant, ali samo za jezike za koje je napisan prompt ovog agenta.

### Zašto ograničiti agenta

- **Troškovi.** Ograničavanje smanjuje broj triggera koje agent mora obraditi, a time i potrošnju tokena.
- **Ispravnost.** Sažimač optimiziran za tehničke članke može dati loše rezultate na stranicama proizvoda. Ograničavanje je precizniji alat nego tražiti od prompta da "preskoči ne-tehničke stranice" na engleskom.
- **Ponašanje specifično za lokalitet.** Pozdravni agent koji piše samo na njemačkom treba se pokretati samo na komentarima s njemačkim lokalitetom. Kombinirajte `de_de` lokalitet s njemačkim tonom u [initial prompt](#personality-prompt).

### Što ograničavanje *ne radi*

- Ne mijenja **agent slot count** (vidi [Plans and Eligibility](#plans-and-eligibility)) - ograničeni agent i dalje zauzima jedan slot.
- Ne mijenja [Budget caps](#budgets-overview) - dnevni i mjesečni ograničenja po agentu vrijede za sve odgovarajuće triggere.
- Ne retroaktivno ograničava prošle izvedbe - povijest rada prikazuje sve što je agent učinio, čak i ako ga naknadno oštrije ograničite.