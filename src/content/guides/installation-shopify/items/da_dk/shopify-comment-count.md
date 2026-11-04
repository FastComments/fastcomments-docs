Blokken **FastComments - Comment Count** viser et lille antal kommentarer for en enkelt side. Brug den i blogindlægslistor, produktkort eller enhver skabelon, der linker til en side med kommentarer, så besøgende kan se, hvor aktiv hver tråd er, før de klikker videre.

### Tilføj blokken

1. Åbn Shopify-temaeditoren.
2. Åbn den skabelon, hvor antallet skal vises. For eksempel skabelonen **Blog** (indlægslisten) eller et produktlistesektion.
3. Klik på **Add block** i den sektion, der viser hvert element.
4. Under **Apps** skal du vælge **FastComments - Comment Count**.
5. Klik på **Save**.

### Indstillinger

| Indstilling | Hvad den gør | Standard |
|---|---|---|
| Tenant ID (valgfri) | Tilsidesæt, hvilken FastComments-tenant antallet læses fra. Lad feltet stå tomt for at bruge butikkens automatisk konfigurerede tenant. | (tom) |
| Custom URL ID | Tilsidesæt det side-id, som antallet slår op. Brug denne, når antallet er på en anden side end den FastComments-blok, den sporer. | (automatisk registreret) |

### Sådan matcher antallet kommentartråden

Comment Count-blokken bruger samme automatisk registreringslogik som **FastComments**-blokken:

- Blogindlægsskabelon: `shopify-article-{article.id}`
- Produktskabelon: `shopify-product-{product.id}`
- Andre skabeloner: anmodningsstien

Hvis du har angivet en **Custom URL ID** på **FastComments**-blokken på en side, skal du angive den samme Custom URL ID på Comment Count-blokken, så de peger på samme tråd.

### Tips

- Antal for hvert element på siden hentes i én anmodning, så det koster ingen ekstra rundtur at tilføje blokken til hvert element i en lang liste.
- Én Comment Count-blok pr. artikel eller produkt i en liste er den forventede brug; blokken kan tilføjes så mange gange, du har brug for.