### Instalirajte iz Shopify App Storea

1. Otvorite [FastComments listing on the Shopify App Store](https://apps.shopify.com/fastcomments).
2. Kliknite **Add app** i odaberite plan koji želite tijekom procesa instalacije.
3. Shopify će vas preusmjeriti natrag u FastComments administraciju unutar Shopifyja kada instalacija završi.

To je cijela instalacija. Nema ništa za lijepljenje u datoteke teme.

### Što se postavlja za vas

Instalacija pokreće sve što biste inače radili ručno:

- Za vašu trgovinu se kreira FastComments tenant i povezuje s vašom domenom trgovine.
- URL vaše trgovine dodaje se u ovlaštene domene tenanta, tako da se komentari učitavaju bez pogreške domene.
- Upisuje se `fastcomments.tenant_id` shop metafield tako da svaki blok zna protiv kojeg tenanta treba renderirati.
- Jedinstvena prijava (Single sign-on) za vaše Shopify kupce je prema zadanim postavkama omogućena.
- Naplate se obavljaju putem Shopify Managed Pricing. Troškovi se pojavljuju na vašem uobičajenom Shopify računu. Nadogradite, degradirajte ili otkažite iz **Settings > Apps and sales channels > FastComments** u vašem Shopify administracijskom sučelju.

Ako je vaša trgovina već bila FastComments korisnik prije nego što ste instalirali aplikaciju, instalacija ponovno koristi postojeći tenant umjesto da kreira novi.

### Ugrađena administratorska ploča

Kada otvorite FastComments aplikaciju iz vašeg Shopify administracijskog sučelja, dođete na nadzornu ploču s pločicama jednim klikom koje vode u puni FastComments backend:

- **Dashboard**: postavke računa, korištenje i pojedinosti o pretplati.
- **Moderation Queue**: odobravanje, odbijanje i odgovaranje na komentare diljem vaše trgovine.
- **Customize**: prilagodite boje widgeta, fontove, pravila moderiranja i konfiguraciju.
- **Ratings & Reviews Helper**: postavite zvjezdane ocjene i pitanja za recenzije ako želite koristiti blok Reviews Summary.

Svaka pločica otvara FastComments s jednokratnom prijavnom poveznicom, pa vam nije potreban zaseban prijavni račun.

### Sljedeće: dodajte blokove u vašu trgovinu

Otvorite uređivač teme u Shopifyju (**Online Store > Themes > Customize**), otvorite predložak u koji želite dodati komentare ili recenzije i kliknite **Add block**. FastComments blokovi pojavljuju se pod **Apps**. Ostatak ovog vodiča obuhvaća svaki od njih.