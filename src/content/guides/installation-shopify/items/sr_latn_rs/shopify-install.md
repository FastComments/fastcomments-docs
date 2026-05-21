### Instalirajte iz Shopify App Store-a

1. Otvorite [FastComments listing on the Shopify App Store](https://apps.shopify.com/fastcomments).
2. Kliknite **Dodaj aplikaciju** i odaberite plan koji želite tokom procesa instalacije.
3. Shopify će vas preusmeriti nazad u FastComments administraciju unutar Shopify-a kada se instalacija završi.

To je cela instalacija. Ne morate ništa lepiti u fajlove vaše teme.

### Šta se podešava za vas

Instalacija izvršava sve što biste inače radili ručno:

- Kreira se FastComments tenant za vašu prodavnicu i povezuje se sa domenom vaše prodavnice.
- URL vaše prodavnice se dodaje u ovlašćene domene tenanta, tako da se komentari učitavaju bez greške domene.
- Upisuje se metapolje prodavnice `fastcomments.tenant_id` tako da svaki blok zna protiv kog tenanta da renderuje.
- Single sign-on za vaše Shopify kupce je podrazumevano omogućen.
- Naplate se vrše preko Shopify Managed Pricing. Troškovi se pojavljuju na vašem redovnom Shopify računu. Nadogradite, promenite plan ili otkažite iz **Podešavanja > Aplikacije i prodajni kanali > FastComments** u vašoj Shopify administraciji.

Ako je vaša prodavnica već bila FastComments korisnik pre nego što ste instalirali aplikaciju, instalacija će ponovo koristiti postojeći tenant umesto da kreira novi.

### Ugrađena administracija

Kada otvorite FastComments aplikaciju iz vaše Shopify administracije, stignete na nadzornu tablu sa pločicama za jedan klik koje vode u kompletan FastComments backend:

- **Nadzorna tabla**: podešavanja naloga, korišćenje i detalji pretplate.
- **Moderation Queue**: odobravanje, odbijanje i odgovaranje na komentare širom vaše prodavnice.
- **Customize**: prilagodite boje widgeta, fontove, pravila moderacije i konfiguraciju.
- **Ratings & Reviews Helper**: podesite ocene zvezdama i pitanja za recenzije ako želite da koristite blok Reviews Summary.

Svaka pločica otvara FastComments sa jednokratnim linkom za prijavu, tako da vam nije potreban poseban nalog za prijavu.

### Sledeće: dodajte blokove u vašu prodavnicu

Otvorite editor teme u Shopify-ju (**Online prodavnica > Teme > Prilagodi**), otvorite template u koji želite da dodate komentare ili recenzije i kliknite **Dodaj blok**. FastComments blokovi se pojavljuju pod **Aplikacije**. Ostatak ovog vodiča pokriva svaki od njih.