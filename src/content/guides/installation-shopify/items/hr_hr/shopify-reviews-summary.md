Blok **FastComments - Reviews Summary** prikazuje agregiranu ocjenu zvjezdicama i pregled recenzija za stranicu. Upotrijebite ga zajedno s blokom **FastComments** na predlošcima proizvoda za standardni raspored recenzija: sažetak gore, obrazac za recenzije i recenzije dolje.

### Preduvjet: postavite Ocjene i recenzije

Blok Reviews Summary prikazuje pitanja za ocjene koja ste konfigurirali za svoju trgovinu. Prvo ih postavite:

1. Otvorite FastComments aplikaciju u svom Shopify adminu.
2. Kliknite pločicu **Ratings & Reviews Helper** (ili izravno otvorite [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Dodajte pitanja na koja želite da svaki recenzent odgovori (ukupna ocjena zvjezdicama, "kako je stajala odjeća" itd.).

Bez konfiguriranih pitanja, blok sažetka nema što agregirati.

### Dodavanje bloka

1. Otvorite Shopify uređivač teme.
2. Otvorite predložak **Product** (ili predložak stranice na kojem želite sažetak).
3. Kliknite **Add block** pri vrhu odjeljka stranice, iznad mjesta gdje će biti blok **FastComments**.
4. Pod **Apps**, odaberite **FastComments - Reviews Summary**.
5. Dodajte blok **FastComments** niže na istoj stranici ako ga već nemate, kako bi posjetitelji mogli ostaviti recenzije.
6. Kliknite **Save**.

### Postavke

| Postavka | Što radi | Zadano |
|---|---|---|
| Tenant ID (neobavezno) | Nadjačajte iz kojeg FastComments tenanta sažetak čita podatke. Ostavite prazno za korištenje automatski konfiguriranog tenanta trgovine. | (prazno) |
| Custom URL ID | Nadjačajte identifikator stranice prema kojem se sažetak agregira. Koristite ovo kada se sažetak nalazi na drugoj stranici od bloka FastComments koji odražava. | (automatski otkriveno) |

### Kako sažetak odgovara recenzijama

Blok Reviews Summary koristi istu logiku automatskog otkrivanja kao blok **FastComments**:

- Predložak proizvoda: `shopify-product-{product.id}`
- Predložak objave na blogu: `shopify-article-{article.id}`
- Ostali predlošci: putanja zahtjeva

Za normalnu stranicu proizvoda, sažetak i niz komentara automatski dijele URL ID, bez ikakve konfiguracije.

### Savjeti

- Sažetak je samo za čitanje. Za prikupljanje recenzija potreban vam je blok **FastComments** na istoj stranici.
- Ako promijenite pitanja za ocjene u Ratings & Reviews Helper nakon prikupljanja recenzija, sažetak se ponovno izračunava prema novom skupu pitanja.