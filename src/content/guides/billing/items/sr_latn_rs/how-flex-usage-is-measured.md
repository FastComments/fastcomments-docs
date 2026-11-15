Flex faktura je lista stavki, jedna za svaku vrstu korišćenja, gde svaka prikazuje količinu i cenu, na primer `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Cene su na [stranici sa cenama](https://fastcomments.com/traffic-pricing) i na vašoj stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Ovo odeljenje objašnjava šta svaka količina znači.

### Broji se tokom meseca

Ove stavke se resetuju svakog meseca i broje se od prvog do poslednjeg dana kalendarskog meseca (UTC):

- **Page Loads**: broj puta kada se vidžet za komentare ili feed učitao na vašim stranicama. Svako učitavanje vidžeta za komentare se broji, bez obzira na broj komentara koje prikazuje. Ne naplaćuje se po niti komentara niti po stranici.
- **API Credits**: pozivi REST API-ja sa vašim API ključem.
- **Small Widgets Credits**: učitavanja malih vidžeta, kao što su brojači komentara i bedževi.
- **LLM Tokens**: tokeni koje koriste [AI Agents](/guide-ai-agents.html) i druge LLM funkcije.

### Broji se na kraju meseca

Ovo je snimak stanja koji se pravi kada se faktura kreira:

- **Comments Stored**: ukupan broj komentara na vašem nalogu, ne samo onih dodatih ovog meseca. Brisanje komentara smanjuje ovaj broj, osim što se izbrisani komentar sa odgovorima zadržava kao rezervisano mesto kako odgovori i dalje imaju smisla.
- **SSO Users**: ukupan broj SSO korisnika koje je vaš sajt kreirao, uključujući i osobe koje su komentarisale samo jednom davno. Ako ovo čini najveći deo vašeg računa, pitajte podršku o naplati po mesečno aktivnim korisnicima, gde se računaju samo korisnici koji su se prijavili tokom meseca.
- **Moderators**, **Admins**, **SSO Moderators** i **SSO Admins**: mesta, pokriveno u odeljku [Admin and Moderator Seats](#admin-and-moderator-seats). Prvi admin je besplatan.
- **Domains**: sajtovi na vašem nalogu. Prvi sajt je besplatan, a svaki dodatni se naplaćuje mesečno, dok god je na nalogu. Poddomen se računa samo ako ga dodate kao zaseban sajt; jedan glavni domen pokriva svoje poddomene. Vidite [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: pod-nalozi koje upravlja ovaj nalog, ako ih ima, plus njihovo sopstveno korišćenje kao zasebne stavke.

### Minimum

Ako stavke u zbiru daju manje od Flex minimuma, faktura se dopunjuje do minimuma. Mali sajt sa jednim adminom i jednim domenom plaća samo minimum.

### Pregled fakture

Stranica [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje svaku stavku sledeće fakture sa današnjim brojkama, tako da nema iznenađenja. Broj učitavanja stranica, API poziva i malih vidžeta može zaostajati do minut; ostalo je u realnom vremenu.

### Druge valute

Flex korišćenje se naplaćuje u američkim dolarima. Ako je valuta vaše fakture neka druga, svaka stavka se konvertuje po kursu na dan kreiranja fakture, tako da isto korišćenje može koštati malo drugačije od meseca do meseca. Vidite [Currencies](#currencies).