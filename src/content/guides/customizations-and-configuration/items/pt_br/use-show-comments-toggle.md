[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments renderiza a caixa de entrada de comentários e a sequência de comentários ao mesmo tempo. Para economizar espaço vertical, ele também oculta quaisquer outros campos obrigatórios até que o widget seja interagido.

No entanto, o widget de comentários pode ser ocultado atrás de um botão, por exemplo:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Widget de comentários recolhido atrás de um botão que mostra a contagem de comentários até que o leitor clique nele'; title='Click to Show Comments' app-screenshot-end]

O botão usa texto traduzido diferente dependendo de os comentários estarem atualmente exibidos ou não. Se os comentários estiverem ocultos, ele usa `translations.SHOW_COMMENTS_BUTTON_TEXT`. Se os comentários estiverem exibidos, ele usa `translations.HIDE_COMMENTS_BUTTON_TEXT`. As traduções podem conter o texto `[count]`, que será substituído pela contagem localizada.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Clique para mostrar ou ocultar comentários'; code-example-end]

Isso foi projetado para substituir a configuração `hideCommentsUnderCountTextFormat`.

A contagem é atualizada em tempo real com a sequência de comentários. O botão não é exibido se não houver comentários.

Isso pode ser habilitado sem código criando uma regra de personalização e ativando "Clique para Mostrar Comentários":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Caixa de seleção \'Clique para mostrar comentários\' marcada em uma regra de personalização na página de personalização do widget'; title='Enable Click to Show Comments' app-screenshot-end]