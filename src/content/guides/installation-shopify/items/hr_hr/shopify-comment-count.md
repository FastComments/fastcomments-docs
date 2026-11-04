Blok **FastComments - Comment Count** prikazuje mali broj komentara za jednu stranicu. Koristite ga u listama blog objava, karticama proizvoda ili bilo kojem predlošku koji povezuje na stranicu s komentarima, kako bi posjetitelji mogli vidjeti koliko je svaka rasprava aktivna prije klika.

### Dodavanje bloka

1. Otvorite Shopify uređivač teme.
2. Otvorite predložak u kojem želite da se broj prikaže. Na primjer, predložak **Blog** (lista objava) ili sekciju s popisom proizvoda.
3. Kliknite **Add block** u sekciji koja prikazuje svaku stavku.
4. Pod **Apps**, odaberite **FastComments - Comment Count**.
5. Kliknite **Save**.

### Postavke

| Postavka | Što radi | Zadano |
|---|---|---|
| Tenant ID (opcionalno) | Nadjačava iz kojeg FastComments tenanta broj čita podatke. Ostavite prazno za korištenje automatski konfiguriranog tenanta trgovine. | (prazno) |
| Custom URL ID | Nadjačava identifikator stranice koji broj traži. Koristite ovo kada je broj na drugačijoj stranici od FastComments bloka koji prati. | (automatski otkriveno) |

### Kako broj odgovara nizu komentara

Blok Comment Count koristi istu logiku automatskog otkrivanja kao blok **FastComments**:

- Predložak blog objave: `shopify-article-{article.id}`
- Predložak proizvoda: `shopify-product-{product.id}`
- Ostali predlošci: putanja zahtjeva

Ako ste postavili **Custom URL ID** na bloku **FastComments** na stranici, postavite isti Custom URL ID i na blok Comment Count kako bi upućivali na isti niz komentara.

### Savjeti

- Brojevi za sve stavke na stranici dohvaćaju se jednim zahtjevom, pa dodavanje bloka na svaku stavku u dugoj listi nema dodatnih troškova mrežnih poziva.
- Jedan blok Comment Count po objavi ili proizvodu u listi je očekivana upotreba; blok se može dodati onoliko puta koliko je potrebno.