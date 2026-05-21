The **FastComments - Comment Count** block prikazuje mali broj komentara za jednu stranicu. Koristite ga na listama blog postova, karticama proizvoda ili bilo kojoj šabloni koja linkuje na stranicu sa komentarima, tako da posetioci mogu videti koliko je aktivna svaka nit pre nego što kliknu.

### Dodajte blok

1. Otvorite Shopify uređivač teme.
2. Otvorite šablon u kome želite da se brojač pojavi. Na primer, šablon **Blog** (lista objava) ili sekcija sa listom proizvoda.
3. Kliknite **Add block** u sekciji koja prikazuje svaki predmet.
4. Pod **Apps**, izaberite **FastComments - Comment Count**.
5. Kliknite **Save**.

### Podešavanja

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Prepišite iz kog FastComments tenant-a se čita broj. Ostavite prazno da biste koristili tenant koji je automatski konfigurisan za prodavnicu. | (prazno) |
| Custom URL ID | Prepišite identifikator stranice koji se pretražuje za broj. Koristite ovo kada je brojač na drugačijoj stranici od FastComments bloka koji prati. | (automatski otkriven) |

### Kako se broj poklapa sa nitima komentara

Comment Count blok koristi istu logiku automatskog otkrivanja kao i **FastComments** blok:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Ako postavite **Custom URL ID** na **FastComments** bloku na nekoj stranici, postavite isti Custom URL ID i na Comment Count bloku tako da pokazuju na istu nit.

### Saveti

- Brojevi za svaki predmet na stranici se dohvataju jednim zahtevom, tako da dodavanje bloka za svaki predmet u dugoj listi nema dodatni trošak po rundi.
- Jedan Comment Count blok po članku ili proizvodu u listi je očekivana upotreba; blok možete dodati onoliko puta koliko vam treba.