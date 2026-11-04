Per impostazione predefinita, la dimensione della pagina di FastComments è `30`. Questo include le risposte nei thread.

La dimensione della pagina può essere personalizzata nella [UI di Configurazione del Widget](https://fastcomments.com/auth/my-account/customize-widget) in varie dimensioni da `10` a `200`.

Nota che la modifica della dimensione della pagina richiede il ricalcolo di tutti i thread di commenti nel tuo account. Questo potrebbe richiedere un paio di minuti.

Non è possibile configurarla lato client nel widget, poiché le pagine vengono calcolate lato server.

Un esempio di configurazione è mostrato di seguito:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; alt='Selettore della dimensione della pagina nella pagina di personalizzazione del widget, dove è possibile scegliere un valore da 10 a 200'; title='Dimensioni di Pagina Personalizzate' app-screenshot-end]

Le dimensioni della pagina possono essere personalizzate a livello globale, per dominio o per pagina, creando diverse regole di personalizzazione.

Questo influenzerà tutti i client, le integrazioni e i framework che potresti utilizzare per mostrare i commenti tramite la nostra piattaforma.