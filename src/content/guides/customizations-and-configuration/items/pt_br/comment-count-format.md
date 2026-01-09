[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

A contagem de comentários exibida no topo do widget de comentários pode ser personalizada.

Isso pode ser substituído por qualquer string, e o valor **[count]** será substituído pelo valor da contagem, localizado para o usuário.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]