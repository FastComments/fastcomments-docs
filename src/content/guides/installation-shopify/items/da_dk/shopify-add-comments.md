The **FastComments** blok er hovedkommentar-widgeten. Tilføj den til blogindlægsskabeloner, produktskabeloner eller enhver anden side, hvor du ønsker en diskussionstråd eller en livechat.

### Tilføj blokken

1. Åbn Shopify-temaeditoren (**Online Store > Themes > Customize**).
2. Vælg den skabelon, du vil have kommentarer på: **Blog post**, **Product**, eller enhver anden side- eller sektionsskabelon.
3. I den sektion, hvor du vil have kommentarer vist, klik **Add block**.
4. Under **Apps**, vælg **FastComments**.
5. Klik **Save**.

Blokken vises med det samme. Der er ingen Tenant ID at indtaste; din butiks tenant er tilsluttet automatisk, når du installerer appen.

### Indstillinger

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Overskriv hvilken FastComments tenant blokken gengiver imod. Lad være tom for at bruge butikkens automatisk-konfigurerede tenant. Find a manual tenant ID at fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Logger automatisk besøgende ind som deres Shopify-kundekonto, før de kommenterer. Se [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** for indlejrede svar og stemmer, eller **Streaming** for et realtids chat-feed. | Threaded |
| Custom URL ID | Overskriv den automatisk opdagede sideidentifikator. Brug dette når du vil have to URL'er til at dele én kommentartråd. | (auto-detected) |

### Hvordan sideidentifikatoren vælges

Hver kommentartråd er nøglebundet til et URL-id. Blokken vælger et automatisk:

- **Blog post template:** `shopify-article-{article.id}`, som er stabil på tværs af slug- og titelændringer.
- **Product template:** `shopify-product-{product.id}`, som er stabil på tværs af slug- og titelændringer.
- **Other templates:** the request path.

Hvis du sætter **Custom URL ID**, bruges den værdi i stedet. Brug den samme Custom URL ID på tværs af flere blokke (for eksempel på en lokaliseret variant af en produktside) for at dele én kommentartråd.

### Threaded vs Streaming

**Threaded** er standard. Besøgende svarer hinanden, stemmer, og moderationværktøjer fungerer som forventet. Bedst til blogindlæg og produktanmeldelser.

**Streaming** fjerner trådstrukturen og viser nye kommentarer i realtid, efterhånden som de postes, ligesom et chat-feed. Bedst til produktlanceringer, live-begivenheder og fællesskabssider.

### Flere blokke på samme side

Blokken kan tilføjes mere end én gang til den samme skabelon. For eksempel en Reviews Summary øverst på en produktside og en FastComments-blok nederst. Blokkene deler et URL-id, så oversigten afspejler kommentarerne nedenfor.

### Tips

- Blokken skjuler sig i temaeditorens forhåndsvisning med en gul meddelelse, hvis den ikke kan finde en tenant. Hvis det vises i din livebutik, geninstaller FastComments-appen.
- På en produktside fungerer FastComments-blokken også som din widget til produktanmeldelser. Kombinér den med **FastComments - Reviews Summary** for et oversigt over stjernebedømmelser øverst på siden.