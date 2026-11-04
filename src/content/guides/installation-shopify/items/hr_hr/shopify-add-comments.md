**FastComments** blok je glavni widget za komentare. Dodajte ga u predloške blog objava, predloške proizvoda ili bilo koju drugu stranicu na kojoj želite nit rasprave ili live chat.

### Dodavanje bloka

1. Otvorite Shopify uređivač teme (**Online Store > Themes > Customize**).
2. Odaberite predložak na kojem želite komentare: **Blog post**, **Product** ili bilo koji drugi predložak stranice ili sekcije.
3. U sekciji gdje želite da se komentari pojave, kliknite **Add block**.
4. Pod **Apps**, odaberite **FastComments**.
5. Kliknite **Save**.

Blok se pojavljuje odmah. Nema Tenant ID-a koji treba unijeti; tenant vaše trgovine automatski se povezuje prilikom instalacije aplikacije.

### Postavke

| Postavka | Što radi | Zadano |
|---|---|---|
| Tenant ID (opcionalno) | Nadjačava protiv kojeg FastComments tenanta se blok prikazuje. Ostavite prazno za korištenje automatski konfiguriranog tenanta trgovine. Ručni tenant ID možete pronaći na fastcomments.com/auth/my-account/api-secret. | (prazno) |
| SSO | Automatski prijavljuje posjetitelja kao njegov Shopify korisnički račun prije komentiranja. Pogledajte [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | Uključeno |
| Stil komentiranja | **Threaded** za ugniježđene odgovore i glasove, ili **Streaming** za chat feed u stvarnom vremenu. | Threaded |
| Custom URL ID | Nadjačava automatski otkriveni identifikator stranice. Koristite ovo kada želite da dva URL-a dijele jednu nit komentara. | (automatski otkriveno) |

### Kako se bira identifikator stranice

Svaka nit komentara ključana je URL ID-om. Blok ga automatski bira:

- **Predložak blog objave:** `shopify-article-{article.id}`, koji je stabilan bez obzira na promjene sluga i naslova.
- **Predložak proizvoda:** `shopify-product-{product.id}`, koji je stabilan bez obzira na promjene sluga i naslova.
- **Ostali predlošci:** putanja zahtjeva.

Ako postavite **Custom URL ID**, ta se vrijednost koristi umjesto nje. Koristite isti Custom URL ID u više blokova (na primjer, na lokaliziranoj varijanti stranice proizvoda) kako biste dijelili jednu nit komentara.

### Threaded vs Streaming

**Threaded** je zadani. Posjetitelji međusobno odgovaraju, glasaju, i alati za moderaciju rade kako se očekuje. Najbolje za blog objave i recenzije proizvoda.

**Streaming** uklanja grananje i prikazuje nove komentare u stvarnom vremenu kako se objavljuju, poput chat feeda. Najbolje za lansiranja proizvoda, događaje uživo i stranice zajednice.

### Više blokova na istoj stranici

Blok se može dodati više puta u isti predložak. Na primjer, Reviews Summary na vrhu stranice proizvoda i FastComments blok na dnu. Blokovi dijele URL ID, pa sažetak odražava komentare ispod.

### Savjeti

- Blok se sam sakriva u pretpregledu uređivača teme s žutim obavijestima ako ne može pronaći tenant. Ako se to pojavi u vašoj živoj trgovini, ponovno instalirajte FastComments aplikaciju.
- Za stranicu proizvoda, FastComments blok služi i kao vaš widget za recenzije proizvoda. Upotrijebite ga s **FastComments - Reviews Summary** za sažetak s ocjenama u zvjezdicama na vrhu stranice.