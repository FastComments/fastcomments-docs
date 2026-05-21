The **FastComments** blok podržava jedinstvenu prijavu (SSO) tako da vaši Shopify kupci mogu komentarisati kao oni sami bez kreiranja odvojenog FastComments naloga.

### Kako funkcioniše

Kada posjetilac koji je prijavljen u vašu prodavnicu otvori stranicu sa **FastComments** blokom:

1. Blok otkriva Shopify `customer` objekat.
2. Šalje ime i email kupca FastComments-u putem potpisanog app proxy zahtjeva.
3. FastComments kreira ili usklađuje korisnika čiji je ključ `shopify-{customerId}`, pa isti kupac uvijek odgovara istom FastComments korisniku kroz sesije i ponovne instalacije.
4. Ime posjetioca se pojavljuje na njihovim komentarima. Ne traži im se ponovna prijava.

Ako posjetilac nije prijavljen u prodavnicu, blok se vraća na anonimno komentarisanje (ili FastComments tok prijave, u zavisnosti od konfiguracije vašeg widgeta).

### Isključivanje SSO-a

SSO je podrazumijevano uključen za svaki **FastComments** blok. Da biste ga isključili na konkretnom bloku:

1. Otvorite Shopify uređivač teme.
2. Otvorite šablon koji sadrži blok i kliknite na blok da ga izaberete.
3. Odznačite **SSO**.
4. Kliknite **Sačuvaj**.

Isključite SSO ako želite da komentatori izaberu zasebni identitet za razgovor. Na primjer, interna stranica zajednice na kojoj osoblje komentariše pod drugim prikaznim imenom.

### Šta FastComments prima

SSO payload koji se šalje za svakog kupca sadrži:

- ID korisnika izveden iz Shopify customer ID-a (`shopify-{customerId}`).
- Email kupca (koristi se za identifikaciju korisnika; ne prikazuje se javno).
- Prikazno ime kupca (koristi se kao ime autora njihovog komentara).

Ne šalju se podaci o istoriji narudžbina, plaćanju ili adresi. Payload je potpisan na serverskoj strani; pretraživač kupca nikada ne vidi vjerodajnice.

### Linkovi za prijavu i odjavu

Kada je SSO uključen, linkovi za prijavu i odjavu u widgetu za komentare upućuju na `/account/login` i `/account/logout`, standardne rute Shopify za naloge kupaca. Nema ništa za konfiguraciju. Linkovi rade za svaku prodavnicu koja ima omogućene korisničke naloge.