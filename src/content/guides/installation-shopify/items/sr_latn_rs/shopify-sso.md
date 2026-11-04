**FastComments** blok podržava jedinstveno prijavljivanje (single sign-on) tako da vaši Shopify kupci mogu komentarisati kao sami sebe, bez kreiranja posebnog FastComments naloga.

### Kako funkcioniše

Kada posetilac koji je prijavljen u vašu prodavnicu otvori stranicu sa **FastComments** blokom:

1. Blok detektuje Shopify `customer` objekat.
2. Šalje ime i email kupca FastComments-u putem potpisanog app proxy zahteva.
3. FastComments kreira ili uparuje korisnika sa ključem `shopify-{customerId}`, tako da se isti kupac uvek mapira na istog FastComments korisnika kroz sesije i ponovne instalacije.
4. Ime posetioca se prikazuje na njegovim komentarima. Njemu se ne traži ponovna prijava.

Ako posetilac nije prijavljen u prodavnicu, blok se vraća na anonimno komentarisanje (ili na FastComments tok prijave, zavisno od konfiguracije vašeg vidžeta).

### Isključivanje SSO-a

SSO je podrazumevano uključen za svaki **FastComments** blok. Da ga isključite na određenom bloku:

1. Otvorite Shopify editor teme.
2. Otvorite šablon koji sadrži blok i kliknite na blok da ga izaberete.
3. Uklonite kvačicu sa **SSO**.
4. Kliknite **Save**.

Isključite SSO ako želite da komentatori izaberu poseban identitet za razgovor. Na primer, interna stranica zajednice gde osoblje komentariše pod drugim imenom za prikaz.

### Šta FastComments prima

SSO payload koji se šalje za svakog kupca sadrži:

- Korisnički ID izveden iz Shopify ID-a kupca (`shopify-{customerId}`).
- Email kupca (koristi se za identifikaciju korisnika; ne prikazuje se javno).
- Ime kupca za prikaz (koristi se kao ime autora komentara).

Ne šalju se podaci o istoriji porudžbina, plaćanjima ili adresama. Payload je potpisan na serverskoj strani; pregledač kupca nikada ne vidi kredencijal.

### Linkovi za prijavu i odjavu

Kada je SSO uključen, linkovi za prijavu i odjavu u vidžetu za komentare upućuju na `/account/login` i `/account/logout`, standardne Shopify rute za kupce. Nema ništa za konfigurisanje. Linkovi rade za svaku prodavnicu sa omogućenim nalozima kupaca.