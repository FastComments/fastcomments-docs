Po defaultu, agent radi preko cijelog vašeg tenanta — svaka stranica, svaki locale. Sekcije **Scope** i **Locales** na formi za uređivanje vam omogućavaju da to suzite.

### Ograničiti na određene stranice

Polje **Restrict to specific pages** prihvata po jedan URL pattern po liniji, u url-pattern glob sintaksi. Agent se izvršava samo na komentarima čiji URL stranice odgovara barem jednom od patterna. Primjeri:

- `/news/*` - bilo koja stranica pod `/news`.
- `/forums/*` - bilo koja stranica pod `/forums`.
- `/blog/2026/*` - bilo koja stranica pod `/blog/2026`.
- (više linija zajedno) - agent se izvršava ako se **bilo koji** pattern poklapa.

Maksimum: 50 patterna po agentu. Patterni moraju biti validni url-pattern globovi — forma odbacuje neispravne sa specifičnom greškom.

Kada je polje **prazno**, agent se izvršava na svakoj stranici u tenant-u.

Kada polje **nije prazno**, agent se ponaša restriktivno: bilo koji okidač čiji komentar nema `urlId` (npr. tenant-level događaji bez konteksta stranice) se preskače. Ovo je namjerno — "scope-ovano na /news/*" ne bi trebalo da tiho padne na opciju "sve".

### Ograničiti na određene lokale

Dual-list picker **Restrict to specific locales** prihvata FastComments locale ID-e (`en_us`, `zh_cn`, `de_de`, itd.). Agent se izvršava samo na komentarima čiji je detektovani locale u odabranom spisku.

Detektovani locale dolazi iz komentara polja `locale`, koje postavlja widget za komentare prilikom postovanja na osnovu locale-a stranice.

Kada **nijedan locale** nije odabran, agent se izvršava za svaki locale.

Kada je **jedan ili više locale-a** odabrano, agent se ponaša restriktivno: okidači bez komentara, ili okidači na komentarima bez polja `locale`, se preskaču.

### Kombinovano ograničavanje

URL i locale filteri se kombinuju logičkim AND. Okidač pokreće agenta samo ako **oba** filtera dozvole.

Korisni obrasci:
- `/news/*` URL pattern + `en_us` locale - samo engleski dio vijesti.
- Nema URL filtera + više locale-a - tenant-wide, ali samo za jezike za koje je prompt agenta pisan.

### Zašto ograničiti agenta

- **Trošak.** Ograničavanje smanjuje obim okidača koje agent mora da procesuira, i time smanjuje potrošnju tokena.
- **Ispravnost.** Prompt za sažimanje koji je podešen za tehničke članke može dati loš rezultat na stranicama proizvoda. Ograničavanje je precizniji alat nego tražiti od prompta da „preskoči netehničke stranice“ na engleskom.
- **Ponašanje specifično za locale.** Pozdravni agent koji piše samo na njemačkom treba da se izvršava samo na komentarima sa njemačkim locale-om. Kombinujte `de_de` locale scope sa njemačkim tonom u [initial prompt](#personality-prompt).

### Šta ograničavanje *ne radi*

- Ne mijenja **agent slot count** (vidi [Plans and Eligibility](#plans-and-eligibility)) — ograničeni agent i dalje zauzima jedan slot.
- Ne mijenja [Budget caps](#budgets-overview) — dnevni i mjesečni limiti po agentu primjenjuju se na sve poklapajuće okidače.
- Ne utiče retroaktivno na prethodna izvršavanja — istorija izvršavanja pokazuje sve što je agent uradio, čak i ako kasnije suzite opseg.