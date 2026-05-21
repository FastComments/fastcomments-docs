The **FastComments - Reviews Summary** block prikazuje agregiranu ocenu zvezdicama i razlaganje recenzija za stranicu. Uparite ga sa blokom **FastComments** na šablonima proizvoda za standardni izgled recenzija: sažetak na vrhu, obrazac za recenzije i recenzije ispod.

### Preduslov: podesite Ratings & Reviews

Blok Reviews Summary prikazuje pitanja za ocenjivanje koja ste konfigurisali za svoju prodavnicu. Prvo podesite ta pitanja:

1. Otvorite FastComments aplikaciju u svom Shopify adminu.
2. Kliknite na pločicu **Ratings & Reviews Helper** (ili otvorite [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) direktno).
3. Dodajte pitanja na koja želite da svaki ocenjivač odgovori (ukupna ocena u zvezdicama, "kako je pristajalo", itd.).

Ako pitanja nisu konfigurisana, blok sažetka nema šta da agregira.

### Dodajte blok

1. Otvorite Shopify uređivač tema.
2. Otvorite šablon **Product** (ili šablon stranice gde želite sažetak).
3. Kliknite **Add block** blizu vrha sekcije stranice, iznad mesta gde će se nalaziti blok **FastComments**.
4. U okviru **Apps**, izaberite **FastComments - Reviews Summary**.
5. Dodajte blok **FastComments** niže na istoj stranici ako to već niste uradili, kako bi posetioci mogli ostavljati recenzije.
6. Kliknite **Save**.

### Podešavanja

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Zameni koji FastComments tenant sažetak koristi. Ostavite prazno da biste koristili tenant koji je automatski konfigurisala prodavnica. | (prazno) |
| Custom URL ID | Zameni identifikator stranice protiv kojeg sažetak agregira. Koristite ovo kada se sažetak nalazi na drugoj stranici od bloka **FastComments** koji prikazuje. | (automatski otkriven) |

### Kako se sažetak poklapa sa recenzijama

Blok Reviews Summary koristi istu logiku automatskog otkrivanja kao blok **FastComments**:

- Šablon proizvoda: `shopify-product-{product.id}`
- Šablon blog posta: `shopify-article-{article.id}`
- Ostali šabloni: putanja zahteva

Za uobičajenu stranicu proizvoda, sažetak i nit komentara automatski dele URL ID, bez potrebe za dodatnom konfiguracijom.

### Saveti

- Sažetak je samo za čitanje. Da biste prikupljali recenzije, potreban vam je blok **FastComments** na istoj stranici.
- Ako promenite pitanja za ocenjivanje u Ratings & Reviews Helper nakon prikupljanja recenzija, sažetak će se preračunati prema novom skupu pitanja.