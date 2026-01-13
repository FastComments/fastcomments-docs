A menor extensão possível seria:

[inline-code-attrs-start title = 'Uma Extensão Simples'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Para este exemplo, salve isto como `my-extension.js`, e torne-o disponível em `https://example.com/my-extension.min.js`.

Esta extensão não faz nada; ao ser carregada, ela busca o objeto de extensão criado pela biblioteca principal de comentários.

Este objeto `Extension` é um singleton e não é compartilhado com outras extensões.

A seguir, para carregar nossa extensão, precisamos informar o widget de comentários sobre ela. Por exemplo:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Para exemplos funcionais, veja a próxima seção.