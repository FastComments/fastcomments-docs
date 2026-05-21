Blok **FastComments - Comment Count** prikazuje mali broj komentara za jednu stranicu. Koristite ga u popisima blog objava, karticama proizvoda ili bilo kojem predlošku koji povezuje na stranicu s komentarima, kako bi posjetitelji mogli vidjeti koliko je svaka tema aktivna prije nego što kliknu.

### Add the block

1. Open the Shopify theme editor.
2. Open the template where you want the count to appear. For example, the **Blog** template (the post list) or a product listing section.
3. Click **Add block** in the section that renders each item.
4. Under **Apps**, select **FastComments - Comment Count**.
5. Click **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Prepišite koji FastComments tenant se koristi za dohvat broja. Ostavite prazno za korištenje automatski konfiguriranog tenanta trgovine. | (prazno) |
| Custom URL ID | Prepišite identifikator stranice koji se koristi za dohvat broja. Koristite ovo kada je broj na drugoj stranici nego FastComments blok kojeg prati. | (automatski otkriveno) |

### How the count matches the comment thread

Blok Comment Count koristi istu logiku automatskog otkrivanja kao i blok **FastComments**:

- Predložak blog objave: `shopify-article-{article.id}`
- Predložak proizvoda: `shopify-product-{product.id}`
- Ostali predlošci: put zahtjeva

Ako postavite **Custom URL ID** na bloku **FastComments** na stranici, postavite isti Custom URL ID na blok Comment Count kako bi upućivali na istu nit.

### Tips

- Brojevi za svaki predmet na stranici dohvaćaju se jednim zahtjevom, tako da dodavanje bloka svakom predmetu u dugom popisu nema dodatnog troška mrežnih zahtjeva.
- Očekivano je korištenje jednog Comment Count bloka po članku ili proizvodu u popisu; blok može biti dodan onoliko puta koliko trebate.