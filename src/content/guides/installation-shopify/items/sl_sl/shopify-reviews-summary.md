Blok **FastComments - Reviews Summary** prikazuje združeno oceno z zvezdicami in razčlenitev ocen za stran. Združite ga z blokom **FastComments** v predlogah izdelkov za standardno postavitev ocen: povzetek zgoraj, obrazec za ocene in ocene spodaj.

### Predpogoj: nastavite Ratings & Reviews

Blok Reviews Summary prikazuje vprašanja za ocenjevanje, ki ste jih nastavili za svojo trgovino. Najprej jih nastavite:

1. Odprite aplikacijo FastComments v svoji Shopify upravni plošči.
2. Kliknite ploščico **Ratings & Reviews Helper** (ali odprite [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) neposredno).
3. Dodajte vprašanja, na katera želite, da vsak ocenitelj odgovori (splošna ocena z zvezdicami, "kako se prilega", itd.).

Brez konfiguriranih vprašanj povzetek nima ničesar za združevanje.

### Dodajte blok

1. Odprite urejevalnik teme Shopify.
2. Odprite predlogo **Product** (ali predlogo strani, kjer želite povzetek).
3. Kliknite **Add block** blizu vrha razdelka strani, nad mestom, kjer bo blok **FastComments**.
4. Pod **Apps** izberite **FastComments - Reviews Summary**.
5. Dodajte spodaj na isti strani blok **FastComments**, če ga še nimate, da lahko obiskovalci pustijo ocene.
6. Kliknite **Save**.

### Nastavitve

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the summary reads from. Leave blank to use the store's automatically-configured tenant. | (blank) |
| Custom URL ID | Override the page identifier the summary aggregates against. Use this when the summary lives on a different page from the FastComments block it reflects. | (auto-detected) |

### Kako se povzetek ujema z ocenami

Blok Reviews Summary uporablja isto logiko samodezne zaznave kot blok **FastComments**:

- Predloga izdelka: `shopify-product-{product.id}`
- Predloga članka: `shopify-article-{article.id}`
- Druge predloge: pot zahteve

Na običajni strani izdelka povzetek in nit komentarjev samodejno uporabljata isti URL ID, brez dodatne konfiguracije.

### Nasveti

- Povzetek je samo za branje. Za zbiranje ocen potrebujete na isti strani blok **FastComments**.
- Če po zbiranju ocen spremenite vprašanja za ocenjevanje v Pomočniku za ocene in mnenja (Ratings & Reviews Helper), se povzetek ponovno izračuna glede na nov nabor vprašanj.