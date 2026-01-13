[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Quando si inviano email di notifica, o si visualizzano i commenti in interfacce utente come la pagina di moderazione, è utile poter collegare
dal commento alla pagina su cui si trova.

Se l'ID dell'URL non è sempre un vero e proprio ID, allora dobbiamo memorizzare l'URL altrove. È per questo che esiste la proprietà "url", definita come segue.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Un caso d'uso comune è collegare il thread dei commenti a un identificatore, come un articolo, e poi rimandare a una pagina specifica, per esempio:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

L'URL non viene ripulito dai comuni parametri di marketing. Per impostazione predefinita, qualunque sia l'URL della pagina corrente, è l'URL che viene memorizzato con il commento.