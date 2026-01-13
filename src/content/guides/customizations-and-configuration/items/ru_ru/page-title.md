[related-parameter-start name = 'pageTitle'; type = 'string'; related-parameter-end]

Текущий заголовок страницы связывается с указанным **urlId** и сохраняется для использования в инструментах модерации.

По умолчанию он получается из **document.title**.

При необходимости вы можете указать собственный заголовок страницы следующим образом:

[code-example-start config = {pageTitle: "Article 42"}; linesToHighlight = [6]; title = 'Specifying The Page Title'; code-example-end]