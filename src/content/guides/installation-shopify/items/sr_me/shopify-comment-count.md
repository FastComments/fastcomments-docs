The **FastComments - Comment Count** block prikazuje mali broj komentara za pojedinačnu stranicu. Koristite ga na listama blog postova, karticama proizvoda ili bilo kojem šablonu koji linkuje na stranicu sa komentarima, tako da posjetioci mogu vidjeti koliko je aktivna svaka nit prije nego što kliknu.

### Add the block

1. Otvorite Shopify theme editor.
2. Otvorite šablon gdje želite da se broj pojavi. Na primjer, **Blog** template (lista postova) ili sekcija za listanje proizvoda.
3. Kliknite **Add block** u sekciji koja renderuje svaki predmet.
4. Pod **Apps**, izaberite **FastComments - Comment Count**.
5. Kliknite **Save**.

### Settings

| Setting | Šta radi | Default |
|---|---|---|
| Tenant ID (optional) | Prepišite iz kojeg FastComments tenant-a se čita broj. Ostavite prazno da bi koristilo tenant koji je automatski konfigurisan za prodavnicu. | (blank) |
| Custom URL ID | Prepišite identifikator stranice koji se koristi za pretragu broja. Koristite ovo kada je broj na drugoj stranici nego FastComments blok koji prati. | (auto-detected) |

### How the count matches the comment thread

Comment Count blok koristi istu logiku automatskog otkrivanja kao **FastComments** blok:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Ako postavite **Custom URL ID** na **FastComments** bloku na nekoj stranici, postavite isti Custom URL ID na Comment Count bloku tako da pokazuju na istu nit.

### Tips

- Brojevi za svaki predmet na stranici se dohvaćaju u jednom zahtjevu, tako da dodavanje bloka za svaki predmet u dugoj listi nema dodatni trošak za round-trip.
- Očekivana upotreba je jedan Comment Count blok po članku ili proizvodu u listi; blok možete dodati onoliko puta koliko želite.