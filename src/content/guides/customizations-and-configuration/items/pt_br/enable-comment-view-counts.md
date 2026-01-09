[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não rastreia quem visualizou cada comentário nem fornece estatísticas sobre isso.

No entanto, podemos ativar esse recurso, e então o sistema começará a rastrear quando cada usuário rolar até um comentário.

Quando isso acontecer, uma contagem ao lado de um ícone de olho exibido em cada comentário será incrementada. A contagem é atualizada ao vivo e abreviada de acordo com a localidade do usuário.

Podemos ativar isso definindo a flag **enableViewCounts** como true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Isto pode ser customizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Rastreamos o id do usuário* que visualizou o comentário, para que, se você visualizar o comentário novamente, ele não seja incrementado. Se você visualizar o comentário novamente após dois anos, a contagem será incrementada novamente.

- *Nota: ou o id de sessão anônima, ou o IP do usuário como um valor hash.

---