[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Ao renderizar um thread de comentários, ou ao deixar um comentário, o FastComments precisa saber a que página, artigo ou produto esses comentários pertencem.

Para isso, usamos algo que chamamos de "URL ID". É um identificador, como uma string ou um número, ou uma URL.

Por padrão, se você não especificar o urlId, ele será a URL da página. Nós pegaremos a URL da página atual e a limparemos para remover quaisquer parâmetros de marketing comuns ou identificadores de rastreamento.

No caso de integrações de terceiros, como WordPress, nosso plugin geralmente usará o identificador que representa a informação atual sendo visualizada como o URL ID, por exemplo o id do artigo/página.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definindo um URL ID Personalizado'; code-example-end]

Uma coisa que frequentemente referenciamos neste documento é a <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interface de Personalização de Widget</a>.

Esta interface pode ser usada para fazer muitas alterações no widget de comentários sem usar código.

Ao criar uma regra de personalização, geralmente queremos que ela se aplique a todas as páginas do nosso site. No entanto, em alguns casos queremos personalizar o widget de comentários em uma página específica, seja para aplicar estilos personalizados ou talvez tornar os comentários daquela página anônimos. Você também poderia, por exemplo, fazer com que comentários ao vivo apareçam imediatamente em algumas páginas, enquanto os oculta sob botões de notificação em outras.

Tudo isso é possível através do campo de entrada URL ID nesta página, que se parece com o seguinte:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Campo URL ID usado para delimitar uma regra de personalização a uma página, ou a um padrão como */blog/*'; title='Entrada URL ID na Página de Personalização do Widget' app-screenshot-end]

O valor neste campo deve corresponder ao parâmetro *urlId* passado para o widget de comentários. Se você quiser que sua regra de personalização seja agnóstica ao *urlId*, deixe este campo vazio ou insira *.

A partir de 2023, o campo `URL ID` na personalização do widget também aceita padrões! Por exemplo, você pode ter `*/blog/*` para adicionar estilos específicos ao seu blog e `*/store/*` para ter estilos específicos à sua loja, tudo usando o mesmo domínio.

### Armadilhas

1. Se sua página tem parâmetros de hash (como example.com#page-1) - isso se tornará parte do URL ID, por padrão.  
2. Durante migrações, por exemplo de WordPress para Gatsby, você pode precisar migrar os valores de comentários do URL ID após a migração inicial. Para isso, entre em contato conosco.

---