[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Ao buscar e renderizar comentários, o widget de comentários precisa saber em qual página começar. Por padrão, ele começa com
a primeira página, renderizando apenas essa página.

Se desejado, a página exata a ser renderizada pode ser passada para o widget de comentários como a configuração *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Observe que os números das páginas começam em zero, então o exemplo acima renderiza a segunda página.