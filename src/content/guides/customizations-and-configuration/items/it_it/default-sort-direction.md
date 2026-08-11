[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Per impostazione predefinita, FastComments ordinerà i commenti secondo la direzione di ordinamento "Most Relevant".

L'ordinamento Most Relevant tiene conto del momento in cui è stato lasciato il commento e del numero di voti per l'ordinamento.

L'utente può quindi cambiare la direzione di ordinamento in Oldest o Newest First nell'interfaccia del widget dei commenti.

Tuttavia, possiamo modificare il valore predefinito in una delle tre opzioni. Per esempio, se vuoi mostrare per primi i commenti più vecchi:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Modifica dell\'ordinamento predefinito in più vecchi per primi'; code-example-end]

Impostiamo il valore di **defaultSortDirection** a "OF" per impostare la direzione su "OF".

Per la direzione di ordinamento newest-first, faremmo quanto segue:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Modifica dell\'ordinamento predefinito in più recenti per primi'; code-example-end]

I valori validi per **defaultSortDirection** sono:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedi la sezione "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Selettore della direzione di ordinamento predefinita che offre Most Relevant, Newest First e Oldest First'; title='Modifica della direzione di ordinamento predefinita' app-screenshot-end]

Nota che i commenti su ogni pagina per ciascuna direzione di ordinamento sono pre‑calcolati, quindi tutte le direzioni di ordinamento hanno le stesse prestazioni.