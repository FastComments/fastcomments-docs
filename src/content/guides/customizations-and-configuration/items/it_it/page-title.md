[related-parameter-start name = 'pageTitle'; type = 'string'; related-parameter-end]

Il titolo della pagina corrente è associato al **urlId** specificato e salvato per l'uso negli strumenti di moderazione.

Per impostazione predefinita, questo viene recuperato da **document.title**.

Se desiderato, è possibile specificare un titolo di pagina personalizzato come segue:

[code-example-start config = {pageTitle: "Article 42"}; linesToHighlight = [6]; title = 'Specifying The Page Title'; code-example-end]