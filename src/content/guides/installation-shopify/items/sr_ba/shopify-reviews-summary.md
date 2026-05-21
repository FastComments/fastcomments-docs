Blok **FastComments - Sažetak recenzija** prikazuje agregiranu ocjenu zvjezdicama i razčlanjivanje recenzija za stranicu. Uparite ga sa **FastComments** blokom na predlošcima proizvoda za standardni izgled recenzija: sažetak na vrhu, obrazac za recenzije i recenzije ispod.

### Preduvjet: postavite Ocjene i recenzije

Blok Sažetak recenzija prikazuje pitanja ocjenjivanja koja ste konfigurirali za vašu trgovinu. Prvo ih podesite:

1. Otvorite FastComments aplikaciju u svojoj Shopify administraciji.
2. Kliknite pločicu **Pomoćnik za ocjene i recenzije** (ili otvorite [Pomoćnik za ocjene i recenzije](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) direktno).
3. Dodajte pitanja na koja želite da svaki recenzent odgovori (ukupna ocjena zvjezdicama, "kako je odgovaralo", itd.).

Bez konfiguriranih pitanja, sažetak nema šta agregirati.

### Dodajte blok

1. Otvorite uređivač teme Shopify.
2. Otvorite predložak **Product** (ili predložak stranice na kojem želite sažetak).
3. Kliknite **Add block** blizu vrha odjeljka stranice, iznad mjesta gdje će biti **FastComments** blok.
4. Pod **Apps**, odaberite **FastComments - Sažetak recenzija**.
5. Dodajte **FastComments** blok niže na istoj stranici ako to već niste učinili, kako bi posjetioci mogli ostaviti recenzije.
6. Kliknite **Save**.

### Postavke

| Postavka | Šta radi | Zadano |
|---|---|---|
| Tenant ID (optional) | Pregazi koji FastComments tenant sažetak čita. Ostavite prazno da biste koristili tenant koji je automatski konfiguriran za trgovinu. | (prazno) |
| Custom URL ID | Pregazi identifikator stranice prema kojem sažetak agregira. Koristite ovo kada sažetak stoji na drugoj stranici od FastComments bloka kojeg odražava. | (automatski otkriveno) |

### Kako se sažetak podudara sa recenzijama

Blok Sažetak recenzija koristi istu logiku automatskog otkrivanja kao i **FastComments** blok:

- Predložak proizvoda: `shopify-product-{product.id}`
- Predložak blog posta: `shopify-article-{article.id}`
- Ostali predlošci: putanja zahtjeva

Za običnu stranicu proizvoda, sažetak i nit komentara automatski dijele ID URL-a, bez potrebe za konfiguracijom.

### Savjeti

- Sažetak je samo za čitanje. Da biste prikupili recenzije, trebate **FastComments** blok na istoj stranici.
- Ako promijenite pitanja za ocjenjivanje u Pomoćniku za ocjene i recenzije nakon prikupljanja recenzija, sažetak će se ponovo izračunati prema novom skupu pitanja.