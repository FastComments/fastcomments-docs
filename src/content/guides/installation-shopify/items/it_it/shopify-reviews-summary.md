Il blocco **FastComments - Riepilogo Recensioni** mostra un punteggio aggregato a stelle e la suddivisione delle recensioni per una pagina. Abbinalo al blocco **FastComments** nei template dei prodotti per il layout standard delle recensioni: riepilogo in alto, modulo per le recensioni e recensioni sotto.

### Prerequisito: configura Valutazioni e Recensioni

Il blocco Riepilogo recensioni mostra le domande di valutazione che hai configurato per il tuo negozio. Configurale prima:

1. Apri l'app FastComments nel tuo pannello di amministrazione Shopify.
2. Clicca sulla tessera **Assistente Valutazioni e Recensioni** (o apri direttamente [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Aggiungi le domande che vuoi che ogni recensore risponda (valutazione complessiva a stelle, "com’è la vestibilità", ecc.).

Senza domande configurate, il blocco riepilogo non ha nulla da aggregare.

### Aggiungi il blocco

1. Apri l'editor del tema di Shopify.
2. Apri il template **Product** (o il template di pagina dove vuoi il riepilogo).
3. Clicca su **Aggiungi blocco** vicino alla parte superiore della sezione pagina, sopra il punto dove sarà il blocco **FastComments**.
4. In **App**, seleziona **FastComments - Riepilogo Recensioni**.
5. Aggiungi un blocco **FastComments** più in basso sulla stessa pagina se non lo hai già fatto, così i visitatori possono lasciare recensioni.
6. Clicca su **Salva**.

### Impostazioni

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Sovrascrive da quale tenant FastComments il riepilogo legge. Lascia vuoto per usare il tenant automaticamente configurato per il negozio. | (vuoto) |
| Custom URL ID | Sovrascrive l'identificatore della pagina su cui il riepilogo si aggrega. Usalo quando il riepilogo si trova in una pagina diversa rispetto al blocco FastComments che riflette. | (rilevato automaticamente) |

### Come il riepilogo corrisponde alle recensioni

Il blocco Riepilogo recensioni usa la stessa logica di rilevamento automatico del blocco **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Per una pagina prodotto normale, il riepilogo e il thread di commenti condividono automaticamente un ID URL, senza necessità di configurazione.

### Suggerimenti

- Il riepilogo è in sola lettura. Per raccogliere recensioni, hai bisogno di un blocco **FastComments** sulla stessa pagina.
- Se modifichi le domande di valutazione nell’Assistente Valutazioni e Recensioni dopo aver raccolto recensioni, il riepilogo si ricalcola rispetto al nuovo set di domande.