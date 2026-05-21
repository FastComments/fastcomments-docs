The **FastComments - Sažetak recenzija** blok prikazuje agregirani prikaz ocjene zvjezdicama i raspodjelu recenzija za stranicu. Uparite ga s blokom **FastComments** na predlošcima proizvoda za standardni izgled recenzija: sažetak pri vrhu, obrazac za recenzije i recenzije ispod.

### Preduvjet: postavite Ratings & Reviews

Blok Sažetak recenzija prikazuje pitanja o ocjenjivanju koja ste konfigurirali za svoju trgovinu. Prvo ih postavite:

1. Otvorite aplikaciju FastComments u svom Shopify administratorskom sučelju.
2. Kliknite pločicu **Pomoćnik za ocjene i recenzije** (ili otvorite [Pomoćnik za ocjene i recenzije](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) izravno).
3. Dodajte pitanja koja želite da svaki recenzent odgovori (ukupna ocjena zvjezdicama, "kako je pristajalo", itd.).

Bez konfiguriranih pitanja, sažetak nema što agregirati.

### Dodavanje bloka

1. Otvorite uređivač teme u Shopifyju.
2. Otvorite predložak **Product** (ili predložak stranice na kojem želite sažetak).
3. Kliknite **Add block** pri vrhu odjeljka stranice, iznad mjesta gdje će biti blok **FastComments**.
4. Pod **Apps**, odaberite **FastComments - Reviews Summary**.
5. Dodajte blok **FastComments** niže na istoj stranici ako ga još nemate, kako bi posjetitelji mogli ostaviti recenzije.
6. Kliknite **Save**.

### Postavke

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the summary reads from. Leave blank to use the store's automatically-configured tenant. | (prazno) |
| Custom URL ID | Override the page identifier the summary aggregates against. Use this when the summary lives on a different page from the FastComments block it reflects. | (automatski otkriveno) |

### Kako se sažetak podudara s recenzijama

Blok Sažetak recenzija koristi istu logiku automatskog otkrivanja kao i blok **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Za uobičajenu stranicu proizvoda, sažetak i niti komentara automatski dijele URL ID, bez potrebe za dodatnom konfiguracijom.

### Savjeti

- Sažetak je samo za čitanje. Da biste prikupljali recenzije, trebate imati blok **FastComments** na istoj stranici.
- Ako promijenite pitanja za ocjenjivanje u Pomoćniku za ocjene i recenzije nakon prikupljanja recenzija, sažetak će se ponovno izračunati prema novom skupu pitanja.