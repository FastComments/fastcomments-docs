The **FastComments** block je glavni widget za komentare. Dodajte ga u predloške objava na blogu, predloške proizvoda ili bilo koju drugu stranicu na kojoj želite thread za raspravu ili live chat.

### Add the block

1. Otvorite uređivač tema Shopifyja (**Online Store > Themes > Customize**).
2. Odaberite predložak na kojem želite komentare: **Blog post**, **Product**, ili bilo koji drugi predložak stranice ili sekcije.
3. U sekciji gdje želite da se komentari pojavljuju, kliknite **Add block**.
4. Pod **Apps**, odaberite **FastComments**.
5. Kliknite **Save**.

Blok se pojavljuje odmah. Nema Tenant ID-a za unos; tenant vaše trgovine je automatski povezan kada instalirate aplikaciju.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (neobavezno) | Override which FastComments tenant the block renders against. Leave blank to use the store's automatically-configured tenant. Find a manual tenant ID at fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Automatski prijavljuje posjetitelja koristeći njihov Shopify korisnički račun prije komentiranja. Pogledajte [Automatsko prijavljivanje Shopify kupaca](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** za ugniježdene odgovore i glasanje, ili **Streaming** za feed chata u stvarnom vremenu. | Threaded |
| Custom URL ID | Override the auto-detected page identifier. Use this when you want two URLs to share one comment thread. | (automatski otkriveno) |

### How the page identifier is chosen

Each comment thread is keyed by a URL ID. The block picks one automatically:

- **Blog post template:** `shopify-article-{article.id}`, which is stable across slug and title changes.
- **Product template:** `shopify-product-{product.id}`, which is stable across slug and title changes.
- **Other templates:** the request path.

If you set **Custom URL ID**, that value is used instead. Use the same Custom URL ID across multiple blocks (for example, on a localized variant of a product page) to share one comment thread.

### Threaded vs Streaming

**Threaded** is the default. Posjetitelji odgovaraju jedni drugima, glasaju, i alati za moderiranje rade kako se očekuje. Najbolje za objave na blogu i recenzije proizvoda.

**Streaming** uklanja ugniježđivanje i prikazuje nove komentare u stvarnom vremenu čim se objave, poput chat feeda. Najbolje za lansiranja proizvoda, live događaje i stranice zajednice.

### Multiple blocks on the same page

Blok se može dodati više puta na isti predložak. Na primjer, sažetak recenzija na vrhu stranice proizvoda i FastComments blok pri dnu. Blokovi dijele URL ID, pa sažetak odražava komentare ispod.

### Tips

- Blok se u pregledniku uređivača teme skriva uz žuto upozorenje ako ne može pronaći tenant. Ako se to pojavi u vašoj live trgovini, ponovno instalirajte FastComments aplikaciju.
- Za stranicu proizvoda, FastComments blok također služi kao widget za recenzije proizvoda. Uparite ga s **FastComments - Reviews Summary** za sažetak ocjena zvjezdicama na vrhu stranice.