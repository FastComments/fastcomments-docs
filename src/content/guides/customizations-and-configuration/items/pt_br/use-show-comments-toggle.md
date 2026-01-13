[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments renderiza a caixa de entrada de comentários e o thread de comentários ao mesmo tempo. Para economizar um pouco de espaço vertical,
ele também ocultará quaisquer outros campos obrigatórios até que haja interação com o widget.

No entanto, o widget de comentários pode ser ocultado atrás de um botão, por exemplo:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

O botão usa textos traduzidos diferentes dependendo se os comentários estão atualmente mostrados ou não. Se os comentários estiverem ocultos, ele usa `translations.SHOW_COMMENTS_BUTTON_TEXT`. Se os
comentários estiverem mostrados, ele usa `translations.HIDE_COMMENTS_BUTTON_TEXT`. As traduções podem conter o texto `[count]` que será
substituído pela contagem localizada.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Isto foi projetado para substituir a configuração `hideCommentsUnderCountTextFormat`.

A contagem é atualizada ao vivo com o thread de comentários. O botão não é exibido se não houver comentários.

Isto pode ser ativado sem código criando uma regra de personalização e habilitando "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]


---