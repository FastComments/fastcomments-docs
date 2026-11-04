Il blocco **FastComments - Comment Count** mostra un piccolo conteggio dei commenti per una singola pagina. Usalo negli elenchi di articoli del blog, nelle schede prodotto o in qualsiasi template che rimanda a una pagina con commenti, così i visitatori possono vedere quanto è attiva ogni discussione prima di cliccare.

### Aggiungere il blocco

1. Apri l'editor del tema Shopify.
2. Apri il template in cui vuoi che appaia il conteggio. Ad esempio, il template **Blog** (l'elenco degli articoli) o una sezione di elenco prodotti.
3. Clicca **Aggiungi blocco** nella sezione che renderizza ogni elemento.
4. Sotto **App**, seleziona **FastComments - Comment Count**.
5. Clicca **Salva**.

### Impostazioni

| Impostazione | Funzione | Predefinito |
|---|---|---|
| Tenant ID (opzionale) | Sovrascrive il tenant FastComments da cui il conteggio legge i dati. Lascia vuoto per usare il tenant configurato automaticamente per lo store. | (vuoto) |
| Custom URL ID | Sovrascrive l'identificatore di pagina che il conteggio cerca. Usalo quando il conteggio si trova su una pagina diversa dal blocco FastComments che traccia. | (rilevato automaticamente) |

### Come il conteggio corrisponde al thread dei commenti

Il blocco Comment Count usa la stessa logica di rilevamento automatico del blocco **FastComments**:

- Template articolo del blog: `shopify-article-{article.id}`
- Template prodotto: `shopify-product-{product.id}`
- Altri template: il percorso della richiesta

Se hai impostato un **Custom URL ID** sul blocco **FastComments** di una pagina, imposta lo stesso Custom URL ID sul blocco Comment Count in modo che puntino allo stesso thread.

### Suggerimenti

- I conteggi per ogni elemento della pagina vengono recuperati in un'unica richiesta, quindi aggiungere il blocco a ogni elemento di un lungo elenco non comporta costi aggiuntivi di round-trip.
- Un blocco Comment Count per articolo o prodotto in un elenco è l'utilizzo previsto; il blocco può essere aggiunto tutte le volte che ti serve.