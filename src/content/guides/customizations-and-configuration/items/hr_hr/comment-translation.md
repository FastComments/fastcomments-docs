---
Prema zadanim postavkama, FastComments prevodi vlastito sučelje widgeta (gumbove, oznake, poruke o pogreškama) na jezik gledatelja, ali se komentari prikazuju točno onako kako su napisani.

Prevođenje komentara dodaje prijevod samih komentara, tako da zajednica koja piše na više jezika može pročitati svaki komentar na svom jeziku.

Postoje dva načina:

- **Automatic**: komentari napisani na jeziku koji se razlikuje od jezika widgeta prikazuju se prevedeni. Linija "Prevedeno s španjolskog" pojavljuje se ispod komentara s poveznicom **Show original** koja prebacuje između prijevoda i neprevedenog teksta.
- **Manual**: svaki komentar dobiva poveznicu **Prevedi**. Klikom na nju taj komentar se prevodi na jezik gledatelja, s istim **Show original** preklopnikom nakon toga. Komentari koji su već na jeziku gledatelja to tako i navode.

Ciljni jezik je jezik widgeta: postavka [Locale / Language](#locale) na istoj pravili prilagodbe ako je postavljena, inače je jezik preglednika posjetitelja. Posjetitelj ne mora ništa, nema potrebe za računom, ekstenzijom ili dodatnim softverom, i radi isto u ugrađenom widgetu i na samostalnim stranicama s komentarima.

Ovo se uključuje po <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagodbe</a>, tako da se može primijeniti na cijelu vašu stranicu, jednu domenu ili pojedinu stranicu. Po zadanim postavkama je isključeno.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Padajući izbornik Prevođenje komentara na stranici prilagodbe widgeta s opcijama Isključeno, Automatski i Ručno'; title='Prevođenje komentara' app-screenshot-end]

Prevođenje je postavka na strani poslužitelja. Prosljeđivanje `commentTranslation` u JavaScript konfiguraciji widgeta ne omogućuje ga; to čini pravilo.

Svaki komentar se prevede najviše jednom po ciljnom jeziku i rezultat se pohranjuje uz komentar, tako da se komentar koji pročita tisuću španjolskih govornika prevede jednom. Ako autor ili moderator uredi komentar, pohranjeni prijevod se odbacuje i komentar se ponovno prevodi pri sljedećem prikazu. Prevedeni tekst prolazi kroz istu sanitaciju i pravila za poveznice kao izvorni komentar. Komentari koji su predugi ili čiji se jezik ne može otkriti prikazuju se neprevedeni.

Korištenje se naplaćuje kao LLM tokeni po istoj stopi kao [AI Agents](/guide-ai-agents.html#plans-and-eligibility) i pojavljuje se na istom mjestu u vašim podacima o naplati.

---