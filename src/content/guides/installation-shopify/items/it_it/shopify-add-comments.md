Il blocco **FastComments** è il widget principale per i commenti. Aggiungilo ai template dei post del blog, ai template dei prodotti o a qualsiasi altra pagina in cui vuoi una discussione o una chat in tempo reale.

### Aggiungi il blocco

1. Apri l'editor del tema di Shopify (**Online Store > Themes > Customize**).
2. Scegli il template sul quale vuoi abilitare i commenti: **Blog post**, **Product**, o qualsiasi altro template di pagina o sezione.
3. Nella sezione in cui vuoi che appaiano i commenti, clicca **Add block**.
4. Sotto **Apps**, seleziona **FastComments**.
5. Clicca **Save**.

Il blocco appare immediatamente. Non c'è nessun Tenant ID da inserire; il tenant del tuo negozio viene configurato automaticamente quando installi l'app.

### Impostazioni

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Sovrascrive quale tenant di FastComments viene usato per il rendering del blocco. Lascia vuoto per usare il tenant configurato automaticamente per il negozio. Trova un Tenant ID manuale su fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Effettua automaticamente il login del visitatore con il suo account cliente Shopify prima che commenti. Vedi [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** per risposte nidificate e voti, oppure **Streaming** per un feed chat in tempo reale. | Threaded |
| Custom URL ID | Sovrascrive l'identificatore di pagina rilevato automaticamente. Usalo quando vuoi che due URL condividano lo stesso thread di commenti. | (auto-detected) |

### Come viene scelto l'identificatore di pagina

Ogni thread di commenti è indicizzato da un URL ID. Il blocco ne seleziona uno automaticamente:

- **Blog post template:** `shopify-article-{article.id}`, che rimane stabile anche se cambiano lo slug o il titolo.
- **Product template:** `shopify-product-{product.id}`, che rimane stabile anche se cambiano lo slug o il titolo.
- **Altri template:** il request path.

Se imposti **Custom URL ID**, viene usato quel valore al posto dell'identificatore automatico. Usa lo stesso Custom URL ID su più blocchi (per esempio, su una variante localizzata di una pagina prodotto) per condividere un unico thread di commenti.

### Threaded vs Streaming

**Threaded** è il valore predefinito. I visitatori si rispondono a vicenda, votano e gli strumenti di moderazione funzionano come previsto. Ideale per post di blog e recensioni di prodotto.

**Streaming** elimina l'annidamento e mostra i nuovi commenti in tempo reale appena vengono pubblicati, come un feed di chat. Ideale per lanci di prodotto, eventi in diretta e pagine community.

### Più blocchi sulla stessa pagina

Il blocco può essere aggiunto più di una volta allo stesso template. Per esempio, un riepilogo delle recensioni in cima a una pagina prodotto e un blocco FastComments in fondo. I blocchi condividono un URL ID, quindi il riepilogo riflette i commenti sottostanti.

### Suggerimenti

- Il blocco si nasconde nella preview dell'editor del tema con una nota gialla se non riesce a trovare un tenant. Se questo appare nel tuo negozio live, reinstalla l'app FastComments.
- Per una pagina prodotto, il blocco FastComments funge anche da widget per le recensioni del prodotto. Abbinalo a **FastComments - Reviews Summary** per un riepilogo delle valutazioni a stelle in cima alla pagina.