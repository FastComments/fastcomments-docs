[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Per impostazione predefinita, FastComments ordina i commenti secondo la direzione di ordinamento "Most Relevant".

L'ordinamento "Most Relevant" tiene conto dell'orario in cui è stato lasciato il commento e del numero di voti per determinare l'ordine.

L'utente può poi cambiare la direzione di ordinamento in "Più vecchi" o "Più recenti per primi" nell'interfaccia del widget dei commenti.

Tuttavia, possiamo cambiare il valore predefinito su una delle tre opzioni. Ad esempio, se si desidera mostrare prima i commenti più vecchi:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Impostiamo il valore di **defaultSortDirection** su "OF" per impostare la direzione su "OF".

Per la direzione di ordinamento "più recenti per primi", faremmo quanto segue:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

I valori validi per **defaultSortDirection** sono:

- MR: "Più recenti"
- NF: "Più recenti per primi"
- OF: "Più vecchi per primi"

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedere la sezione "Direzione di ordinamento predefinita".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Nota che i commenti su ogni pagina per ogni direzione di ordinamento sono pre-calcolati, quindi tutte le direzioni di ordinamento hanno le stesse prestazioni.