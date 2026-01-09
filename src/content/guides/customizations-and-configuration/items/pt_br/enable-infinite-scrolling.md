[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Por padrão, o widget do FastComments redimensiona-se verticalmente para ajustar todos os comentários visíveis. A paginação é feita através de um botão "Ver próximos"
ao final da página atual, pois descobrimos que essa é a interação que mais agrada a maioria dos usuários.

No entanto, existem alguns casos em que a rolagem infinita é preferida. Por exemplo, usamos esse recurso em nosso produto Stream Chat.

Podemos ocultar os botões "Ver próximos" e alternar para rolagem infinita definindo a flag **enableInfiniteScrolling** como true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Isso também requer a adição de CSS personalizado. Adicione CSS personalizado para o seletor `.comments` para habilitar a rolagem, por exemplo:

[inline-code-attrs-start title = 'Ativar Rolagem Infinita'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Um exemplo completo e funcional seria:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

No exemplo acima usamos a propriedade `customCSS`, no entanto, é sugerido usar a UI de Configuração do Widget em vez disso por razões de desempenho. [Veja a documentação de CSS personalizado.](/guide-customizations-and-configuration.html#custom-css)