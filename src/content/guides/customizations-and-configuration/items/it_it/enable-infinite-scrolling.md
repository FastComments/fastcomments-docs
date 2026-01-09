[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, il widget FastComments si ridimensiona verticalmente per adattarsi a tutti i commenti visibili. La paginazione avviene tramite un pulsante "Mostra altri"
alla fine della pagina corrente, poiché abbiamo riscontrato che questa interazione risulta la più naturale per la maggior parte degli utenti.

Tuttavia, ci sono alcuni casi in cui è preferibile lo scorrimento infinito. Ad esempio, utilizziamo questa funzionalità nel nostro prodotto Stream Chat.

Possiamo nascondere i pulsanti "Mostra altri" e passare allo scorrimento infinito impostando il flag **enableInfiniteScrolling** su true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Ciò richiede anche l'aggiunta di CSS personalizzato. Aggiungi CSS personalizzato per il selettore `.comments` per abilitare lo scorrimento, ad esempio:

[inline-code-attrs-start title = 'Abilita Scorrimento Infinito'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Un esempio completo e funzionante sarebbe:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Nell'esempio sopra utilizziamo la proprietà `customCSS`, tuttavia si consiglia di utilizzare invece l'interfaccia di configurazione del Widget per motivi di prestazioni. [Consulta la documentazione sul Custom CSS.](/guide-customizations-and-configuration.html#custom-css)