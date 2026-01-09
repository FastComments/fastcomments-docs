[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Ao renderizar um tópico de comentários, ou ao deixar um comentário, o FastComments precisa saber a que página, artigo ou produto
esses comentários pertencem.

Para isso, usamos algo que chamamos de "URL ID". É ou um identificador, como uma string ou um número, ou uma URL.

Por padrão, se você não especificar o urlId, ele se tornará a URL da página. Nós pegaremos a URL atual da página e a limparemos para remover
quaisquer parâmetros comuns de marketing ou identificadores de rastreamento.

No caso de integrações de terceiros, como WordPress, nosso plugin geralmente usará o identificador que representa a informação atual sendo visualizada como
o URL ID, por exemplo o id do artigo/página.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Uma coisa que frequentemente mencionaremos neste documento é a <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>.

Essa interface pode ser usada para fazer muitas alterações no widget de comentários sem usar código.

Ao criar uma regra de personalização, frequentemente queremos que ela se aplique a todas as páginas do nosso site. No entanto, em alguns casos queremos personalizar o widget de comentários
em uma página específica, seja para aplicar estilos personalizados, ou talvez tornar os comentários daquela página anônimos. Você também poderia, por exemplo, fazer com que comentários ao vivo
apareçam imediatamente em algumas páginas, enquanto em outras fiquem ocultos sob botões de notificação.

Tudo isso é possível através do campo de entrada URL ID nesta página, que se parece com o seguinte:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

O valor neste campo deve corresponder ao parâmetro *urlId* passado para o widget de comentários. Se você quiser que sua regra de personalização seja agnóstica ao *urlId*, deixe este campo vazio ou insira *.

A partir de 2023 o campo `URL ID` na customização do widget também aceita padrões! Por exemplo você pode
usar `*/blog/*` para adicionar estilos específicos ao seu blog e `*/store/*` para ter estilos específicos para sua loja,
tudo isso usando o mesmo domínio.

### Observações

1. Se sua página tem parâmetros de hash (como example.com#page-1) - isso se tornará parte do URL ID por padrão.
2. Durante migrações, por exemplo de WordPress para Gatsby, você pode ter que migrar os valores de comentário do URL ID após a migração inicial. Para isso, entre em contato conosco.