The **FastComments - Comment Count** block renders a small comment count for a single page. Use it in blog post lists, product cards, or any template that links to a page with comments, so visitors can see how active each thread is before clicking through.

### Tilføj blokken

1. Åbn Shopify-temaeditoren.
2. Åbn den skabelon, hvor du vil have tællingen til at vises. For eksempel skabelonen **Blog** (postlisten) eller en produktliste-sektion.
3. Klik på **Add block** i den sektion, der gengiver hvert element.
4. Under **Apps**, vælg **FastComments - Comment Count**.
5. Klik på **Save**.

### Indstillinger

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Overskriv hvilken FastComments tenant tællingen læser fra. Lad stå tomt for at bruge butikkens automatisk konfigurerede tenant. | (tomt) |
| Custom URL ID | Overskriv sideidentifikatoren som tællingen slår op. Brug dette når tællingen er på en anden side end den FastComments blok, den følger. | (automatisk fundet) |

### Hvordan tællingen matcher kommentartråden

Comment Count-blokken bruger den samme auto-detekteringslogik som **FastComments**-blokken:

- Blogindlægsskabelon: `shopify-article-{article.id}`
- Produktskabelon: `shopify-product-{product.id}`
- Andre skabeloner: anmodningsstien

Hvis du angiver en **Custom URL ID** på **FastComments**-blokken på en side, angiv den samme Custom URL ID på Comment Count-blokken, så de peger på samme tråd.

### Tips

- Antallet for hvert element på siden hentes i én forespørgsel, så det at tilføje blokken til hvert element i en lang liste ikke medfører nogen ekstra netværksrundtur.
- Én Comment Count-blok per artikel eller produkt i en liste er den forventede brug; blokken kan tilføjes så mange gange, du har brug for.