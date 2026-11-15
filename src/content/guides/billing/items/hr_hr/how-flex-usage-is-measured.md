Flex faktura je popis stavki, jedna po vrsti korištenja, gdje svaka prikazuje količinu i cijenu, na primjer `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Cijene su na [stranici s cijenama](https://fastcomments.com/traffic-pricing) i na vašoj stranici [Informacije o naplati](https://fastcomments.com/auth/my-account/billing-info). Ovaj dio objašnjava što svaka količina znači.

### Broji se tijekom mjeseca

Ove se vrijednosti resetiraju svaki mjesec i broje se od prvog do posljednjeg dana kalendarskog mjeseca (UTC):

- **Page Loads**: broj učitavanja widgeta za komentare ili feeda na vašim stranicama. Svako učitavanje widgeta za komentare se broji, bez obzira na broj prikazanih komentara. Ne naplaćuje se po niti komentara ni po stranici.
- **API Credits**: pozivi REST API-ja s vašim API ključem.
- **Small Widgets Credits**: učitavanja malih widgeta, poput brojača komentara i znački.
- **LLM Tokens**: tokeni koje koriste [AI agenti](/guide-ai-agents.html) i druge LLM značajke.

### Broji se na kraju mjeseca

Ovo su snimke stanja zabilježene pri stvaranju fakture:

- **Comments Stored**: ukupan broj komentara u vašem računu, ne samo onih dodanih ovog mjeseca. Brisanje komentara smanjuje taj broj, osim što se izbrisani komentar s odgovorima zadržava kao mjesto-držač kako odgovori i dalje imaju smisla.
- **SSO Users**: ukupan broj SSO korisnika koje je vaša stranica stvorila, uključujući osobe koje su komentirale samo davno. Ako ovo čini najveći dio vašeg računa, pitajte podršku o naplati prema mjesečno aktivnim korisnicima, gdje se računaju samo korisnici koji su se prijavili tijekom mjeseca.
- **Moderatori**, **Admini**, **SSO moderatori** i **SSO admini**: mjesta, opisano u [Admin i moderator mjesta](#admin-and-moderator-seats). Prvi admin je besplatan.
- **Domene**: stranice u vašem računu. Prva stranica je besplatna, a svaka dodatna se naplaćuje mjesečno, dok god je u računu. Poddomena se računa samo ako je dodate kao zasebnu stranicu; jedna domena najviše razine pokriva svoje poddomene. Pogledajte [Više stranica](/guide-multiple-sites.html#domain-limits).
- **Upravljani zakupci**: podračuni kojima upravlja ovaj račun, ako ih ima, plus njihovo vlastito korištenje kao zasebne stavke.

### Minimum

Ako stavke zbroje manje od Flex minimuma, faktura se nadopunjuje do minimuma. Mala stranica s jednim adminom i jednom domenom plaća samo minimum.

### Pregled fakture

Stranica [Analitika naplate](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje svaku stavku sljedeće fakture s današnjim brojkama, tako da nema iznenađenja. Broj učitavanja stranica, API poziva i malih widgeta može kasniti do minutu; ostalo je u stvarnom vremenu.

### Ostale valute

Flex korištenje se cijeni u američkim dolarima. Ako je valuta vaše fakture neka druga, svaka se stavka pretvara po tečaju na dan stvaranja fakture, pa isto korištenje može koštati malo drugačije od mjeseca do mjeseca. Pogledajte [Valute](#currencies).