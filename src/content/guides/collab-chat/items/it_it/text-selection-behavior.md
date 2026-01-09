### Come funziona la selezione del testo

Quando gli utenti selezionano del testo all'interno del contenitore Collab Chat, il widget cattura quella selezione e permette loro di avviare una discussione. La selezione può essere tanto piccola quanto una singola parola o tanto estesa quanto più paragrafi che attraversano elementi diversi.

### Ritardo nella selezione

Su dispositivi desktop, c'è un ritardo di 3,5 secondi tra il momento in cui un utente seleziona del testo e il momento in cui appare l'invito alla discussione. Questo evita che l'interfaccia utente si sfarfalli quando gli utenti stanno semplicemente evidenziando del testo per copiarlo o leggerlo. Su dispositivi mobili, l'invito appare immediatamente poiché la selezione del testo è più deliberata sui touchscreen.

### ID conversazione unici

Ogni conversazione ottiene un `urlId` unico che combina l'URL della pagina, il percorso dell'elemento DOM e l'intervallo di testo serializzato. Questo assicura che ogni selezione di testo crei una conversazione distinta che può essere ritrovata in seguito.

Se fornisci un `urlId` personalizzato nella tua configurazione, verrà combinato con il percorso dell'elemento e l'intervallo di testo per creare l'identificatore finale.

### Evidenziazioni visive

Quando esiste una discussione per una particolare selezione di testo, quel testo riceve un'evidenziazione visiva. L'evidenziazione è implementata usando colori di sfondo e compare al passaggio del mouse o quando la finestra di chat associata è aperta.

Il sistema di evidenziazione funziona avvolgendo il testo selezionato in un elemento speciale che può essere stilizzato. Questo approccio assicura che le evidenziazioni rimangano accurate anche quando la struttura HTML sottostante è complessa.

### Posizionamento della finestra di chat

Quando un utente clicca su un'evidenziazione o crea una nuova annotazione, una finestra di chat appare vicino al testo selezionato. Il widget calcola automaticamente la posizione migliore per questa finestra in base allo spazio disponibile nella viewport.

Il sistema di posizionamento usa classi CSS come `to-right`, `to-left`, `to-top` e `to-bottom` per indicare in quale direzione la finestra di chat dovrebbe estendersi dall'evidenziazione. Su dispositivi mobili (schermi con larghezza inferiore a 768px), la finestra di chat appare sempre a schermo intero per una migliore usabilità.

### Dimensioni della finestra di chat

Le finestre di chat hanno larghezza di 410px su desktop con uno spazio di 20px e una freccia visiva di 16px che punta al testo evidenziato. Queste dimensioni sono fisse per garantire un comportamento coerente, ma è possibile personalizzare l'aspetto tramite CSS.

### Selezioni attraverso gli elementi

Gli utenti possono selezionare del testo che attraversa più elementi HTML, ad esempio evidenziando dal mezzo di un paragrafo fino all'inizio di un altro. Il sistema di serializzazione dell'intervallo gestisce correttamente questi casi e evidenzierà tutto il testo selezionato anche attraverso i confini degli elementi.

### Compatibilità del browser

Il sistema di selezione del testo utilizza l'API standard `window.getSelection()` che è supportata in tutti i browser moderni. Per le versioni più vecchie di Internet Explorer, ricorre a `document.selection` per compatibilità.

### Persistenza della selezione

Una volta che una conversazione è stata creata per una selezione di testo, quell'annotazione persiste anche se la pagina viene ricaricata. L'intervallo serializzato e il percorso DOM permettono al widget di ripristinare le evidenziazioni esattamente nello stesso punto quando gli utenti ritornano sulla pagina.

Questo funziona in modo affidabile finché il contenuto della tua pagina rimane stabile. Se modifichi il contenuto testuale o ristrutturi l'HTML, le annotazioni esistenti potrebbero non allinearsi più correttamente con il testo. Per questo motivo, è meglio evitare cambiamenti importanti del contenuto sulle pagine con annotazioni attive, o considerare la migrazione delle annotazioni quando i cambiamenti di contenuto sono necessari.