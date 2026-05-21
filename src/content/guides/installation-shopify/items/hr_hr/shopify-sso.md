The **FastComments** block podržava single sign-on (SSO) tako da vaši Shopify kupci mogu komentirati kao oni sami bez stvaranja zasebnog FastComments računa.

### Kako to funkcionira

Kad posjetitelj koji je prijavljen u vašu trgovinu otvori stranicu s **FastComments** blockom:

1. Blok otkriva Shopify `customer` objekt.
2. Šalje ime i e-poštu kupca FastCommentsu putem potpisanog app proxy zahtjeva.
3. FastComments stvara ili podudara korisnika čiji je ključ `shopify-{customerId}`, tako da isti kupac uvijek mapira na istog FastComments korisnika kroz sesije i ponovne instalacije.
4. Ime posjetitelja pojavljuje se na njihovim komentarima. Ne traži im se ponovno prijavljivanje.

Ako posjetitelj nije prijavljen u trgovinu, blok prelazi na anonimno komentiranje (ili na FastComments tijek prijave, ovisno o konfiguraciji widgeta).

### Isključivanje SSO-a

SSO je uključen prema zadanim postavkama za svaki blok **FastComments**. Da biste ga isključili na određenom bloku:

1. Otvorite Shopify uređivač tema.
2. Otvorite predložak koji sadrži blok i kliknite blok da biste ga odabrali.
3. Odznačite **SSO**.
4. Kliknite **Spremi**.

Isključite SSO ako želite da komentatori odaberu zasebni identitet za razgovor. Na primjer, interna stranica zajednice gdje osoblje komentira pod drugim prikaznim imenom.

### Što FastComments prima

SSO payload poslan za svakog kupca sadrži:

- ID korisnika izveden iz Shopify customer ID-a (`shopify-{customerId}`).
- E-adresu kupca (koristi se za identifikaciju korisnika; ne prikazuje se javno).
- Prikazno ime kupca (koristi se kao ime autora komentara).

Ne šalju se podaci o povijesti narudžbi, plaćanju ili adresi. Payload je potpisan na strani poslužitelja; preglednik kupca nikada ne vidi vjerodajnicu.

### Poveznice za prijavu i odjavu

Kada je SSO uključen, poveznice za prijavu i odjavu u widgetu za komentare vode na `/account/login` i `/account/logout`, standardne Shopify rute za račun kupca. Nije potrebno ništa konfigurirati. Poveznice rade za svaku trgovinu s omogućenim korisničkim računima.