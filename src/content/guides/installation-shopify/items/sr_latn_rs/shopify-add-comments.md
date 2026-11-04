Blok **FastComments** je glavni vidžet za komentare. Dodajte ga u šablone blog objava, šablone proizvoda ili bilo koju drugu stranicu gde želite nit diskusije ili live chat.

### Dodavanje bloka

1. Otvorite Shopify editor teme (**Online Store > Themes > Customize**).
2. Izaberite šablon na kome želite komentare: **Blog post**, **Product** ili bilo koji drugi šablon stranice ili sekcije.
3. U sekciji gde želite da se komentari pojave, kliknite **Add block**.
4. Pod **Apps**, izaberite **FastComments**.
5. Kliknite **Save**.

Blok se pojavljuje odmah. Nema potrebe za unosom Tenant ID-a; tenant vaše prodavnice se povezuje automatski kada instalirate aplikaciju.

### Podešavanja

| Podešavanje | Šta radi | Podrazumevano |
|---|---|---|
| Tenant ID (opciono) | Određuje protiv kog FastComments tenanta će se blok prikazivati. Ostavite prazno da biste koristili automatski konfigurisan tenant prodavnice. Ručni tenant ID možete pronaći na fastcomments.com/auth/my-account/api-secret. | (prazno) |
| SSO | Automatski prijavljuje posetioca kao njegov Shopify korisnički nalog pre komentara. Pogledajte [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | Uključeno |
| Commenting Style | **Threaded** za ugnježdene odgovore i glasanje, ili **Streaming** za feed live chata u realnom vremenu. | Threaded |
| Custom URL ID | Zamenjuje automatski detektovani identifikator stranice. Koristite ovo kada želite da dva URL-a dele jednu nit komentara. | (automatski detektovano) |

### Kako se bira identifikator stranice

Svaka nit komentara je vezana za URL ID. Blok ga bira automatski:

- **Šablon blog objave:** `shopify-article-{article.id}`, koji je stabilan bez obzira na promene slug-a i naslova.
- **Šablon proizvoda:** `shopify-product-{product.id}`, koji je stabilan bez obzira na promene slug-a i naslova.
- **Ostali šabloni:** putanja zahteva.

Ako postavite **Custom URL ID**, ta vrednost se koristi umesto nje. Koristite isti Custom URL ID u više blokova (na primer, na lokalizovanoj varijanti stranice proizvoda) da biste delili jednu nit komentara.

### Threaded vs Streaming

**Threaded** je podrazumevano. Posetioci odgovaraju jedni drugima, glasaju, i alati za moderaciju rade kako se očekuje. Najbolje za blog objave i recenzije proizvoda.

**Streaming** uklanja nit i prikazuje nove komentare u realnom vremenu kako se objavljuju, kao chat feed. Najbolje za lansiranja proizvoda, live događaje i stranice zajednice.

### Više blokova na istoj stranici

Blok se može dodati više puta u isti šablon. Na primer, Reviews Summary na vrhu stranice proizvoda i FastComments blok na dnu. Blokovi dele URL ID, tako da rezime odražava komentare ispod.

### Saveti

- Blok se sakriva u pregledu editora teme sa žutim obaveštenjem ako ne može da pronađe tenant. Ako se to pojavi u vašoj prodavnici, reinstalirajte FastComments aplikaciju.
- Za stranicu proizvoda, FastComments blok služi i kao vidžet za recenzije proizvoda. Upotrebite ga zajedno sa **FastComments - Reviews Summary** za rezime sa zvezdicama na vrhu stranice.