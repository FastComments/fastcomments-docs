[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não rastreia quem visualizou cada comentário nem fornece quaisquer estatísticas sobre isso.

No entanto, podemos habilitar esse recurso, e então o sistema começará a rastrear à medida que cada usuário rola até um comentário.

Quando isso acontece, uma contagem ao lado de um ícone de olho exibido em cada comentário será incrementada. A contagem é atualizada em tempo real e abreviada de acordo com o locale do usuário.

Podemos habilitar isso definindo a flag **enableViewCounts** como true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Habilitando Contagem de Visualizações de Comentários'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Página de personalização do widget com a caixa de seleção de contagem de visualizações marcada, de modo que cada comentário mostra um ícone de olho e a contagem'; title='Habilitando Contagem de Visualizações de Comentários' app-screenshot-end]

Rastreamos o ID do usuário* que visualizou o comentário, de modo que, se você visualizar o comentário novamente, ele não será incrementado. Se você visualizar o comentário novamente
após dois anos, a contagem será incrementada mais.

- *Nota: ou o ID da sessão anônima, ou o IP do usuário como um valor hash.

---