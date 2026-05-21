---
Blok **FastComments - Comment Count** prikaže majhen števec komentarjev za posamezno stran. Uporabite ga v seznamih objav na blogu, karticah izdelkov ali katerikoli predlogi, ki se povezuje s stranjo s komentarji, da lahko obiskovalci vidijo, kako aktivna je posamezna nit, preden kliknejo naprej.

### Dodajte blok

1. Odprite urejevalnik teme Shopify.
2. Odprite predlogo, kjer želite, da se števec prikaže. Na primer predloga **Blog** (seznam objav) ali odsek s seznamom izdelkov.
3. Kliknite **Dodaj blok** v odseku, ki upodablja vsak element.
4. V razdelku **Aplikacije** izberite **FastComments - Comment Count**.
5. Kliknite **Shrani**.

### Nastavitve

| Nastavitev | Kaj počne | Privzeto |
|---|---|---|
| Tenant ID (optional) | Prepiše, pri katerem FastComments najemniku števec bere. Pustite prazno, da se uporabi najemnik, ki ga je trgovina samodejno konfigurirala. | (prazno) |
| Custom URL ID | Prepiše identifikator strani, ki ga števec poišče. Uporabite to, kadar je števec na drugi strani kot FastComments blok, ki ga spremlja. | (samodejno zaznano) |

### Kako se števec ujema z nitjo komentarjev

Blok Comment Count uporablja enako logiko samodejnega odkrivanja kot blok **FastComments**:

- Predloga objave na blogu: `shopify-article-{article.id}`
- Predloga izdelka: `shopify-product-{product.id}`
- Druge predloge: pot zahtevka

Če na strani nastavite **Custom URL ID** v bloku **FastComments**, nastavite isti **Custom URL ID** tudi v bloku Comment Count, da bosta kazala na isto nit.

### Nasveti

- Števila za vsak element na strani se pridobijo z enim zahtevkom, zato dodajanje bloka za vsak element v dolgem seznamu ne povzroča dodatnih krožnih zahtev.
- Pričakovana uporaba je en Comment Count block na članek ali izdelek v seznamu; blok lahko dodate tolikokrat, kot potrebujete.

---