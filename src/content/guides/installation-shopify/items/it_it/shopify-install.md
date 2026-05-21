### Installa dall'App Store di Shopify

1. Apri la [pagina di FastComments sull'App Store di Shopify](https://apps.shopify.com/fastcomments).
2. Clicca **Add app** e scegli il piano che desideri durante il flusso di installazione.
3. Shopify ti reindirizza nell'admin di FastComments all'interno di Shopify quando l'installazione è completata.

Questo è tutto per l'installazione. Non c'è nulla da incollare nei file del tuo tema.

### Cosa viene configurato per te

L'installazione esegue tutto ciò che altrimenti faresti manualmente:

- Viene creato un tenant FastComments per il tuo negozio e collegato al dominio del tuo shop.
- L'URL del tuo negozio viene aggiunto ai domini autorizzati del tenant, così i commenti vengono caricati senza errori di dominio.
- Viene scritto un metafield del negozio `fastcomments.tenant_id` così ogni blocco sa contro quale tenant renderizzare.
- Il single sign-on per i tuoi clienti Shopify è abilitato per impostazione predefinita.
- La fatturazione avviene tramite Shopify Managed Pricing. Gli addebiti compaiono sul tuo consueto estratto conto Shopify. Effettua upgrade, downgrade o cancellazione da **Settings > Apps and sales channels > FastComments** nel pannello di amministrazione di Shopify.

Se il tuo negozio era già cliente FastComments prima di installare l'app, l'installazione riutilizza il tenant esistente invece di crearne uno nuovo.

### L'admin incorporato

Quando apri l'app FastComments dal pannello di Shopify, arrivi in una dashboard con riquadri ad accesso singolo che portano al backend completo di FastComments:

- **Dashboard**: impostazioni account, utilizzo e dettagli dell'abbonamento.
- **Moderation Queue**: approva, rifiuta e rispondi ai commenti in tutto il negozio.
- **Customize**: regola colori del widget, font, regole di moderazione e configurazione.
- **Ratings & Reviews Helper**: configura le valutazioni a stelle e le domande di recensione se vuoi usare il blocco Reviews Summary.

Ogni riquadro apre FastComments con un link di accesso ad uso singolo, quindi non è necessario un accesso separato.

### Successivo: aggiungi blocchi al tuo negozio

Apri l'editor del tema di Shopify (**Negozio online > Temi > Personalizza**), apri il template al quale vuoi aggiungere commenti o recensioni e clicca **Add block**. I blocchi FastComments appaiono sotto **App**. Il resto di questa guida copre ciascuno di essi.