---
Il blocco **FastComments - Comment Count** visualizza un piccolo conteggio dei commenti per una singola pagina. Usalo nelle liste di post del blog, nelle schede prodotto o in qualsiasi template che colleghi a una pagina con commenti, così i visitatori possono vedere quanto è attiva ogni discussione prima di aprirla.

### Add the block

1. Apri l'editor del tema di Shopify.
2. Apri il template in cui vuoi che appaia il conteggio. Per esempio, il template **Blog** (l'elenco dei post) o una sezione di elenco prodotti.
3. Clicca su **Aggiungi blocco** nella sezione che visualizza ogni elemento.
4. Sotto **Apps**, seleziona **FastComments - Comment Count**.
5. Clicca su **Salva**.

### Impostazioni

| Impostazione | Cosa fa | Predefinito |
|---|---|---|
| ID tenant (opzionale) | Sovrascrive da quale tenant FastComments il conteggio legge i dati. Lascia vuoto per usare il tenant configurato automaticamente per il negozio. | (vuoto) |
| ID URL personalizzato | Sovrascrive l'identificatore di pagina che il conteggio cerca. Usalo quando il conteggio si trova in una pagina diversa rispetto al blocco **FastComments** che monitora. | (rilevato automaticamente) |

### Come il conteggio corrisponde alla discussione dei commenti

Il blocco Comment Count utilizza la stessa logica di rilevamento automatico del blocco **FastComments**:

- Template dei post del blog: `shopify-article-{article.id}`
- Template prodotto: `shopify-product-{product.id}`
- Altri template: il percorso della richiesta

Se imposti un **ID URL personalizzato** sul blocco **FastComments** in una pagina, imposta lo stesso **ID URL personalizzato** sul blocco Comment Count affinché puntino alla stessa discussione.

### Suggerimenti

- I conteggi per ogni elemento della pagina vengono recuperati con una sola richiesta, quindi aggiungere il blocco a ogni elemento in una lista lunga non comporta richieste aggiuntive.
- L'uso previsto è un blocco Comment Count per articolo o prodotto in un elenco; il blocco può essere aggiunto tutte le volte che ti serve.

---