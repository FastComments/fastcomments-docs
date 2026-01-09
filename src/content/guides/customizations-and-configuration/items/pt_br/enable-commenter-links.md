[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments pedirá apenas o comentário do usuário, o nome do usuário e o e-mail.

No entanto, em algumas situações você pode querer que o usuário deixe um link para seu próprio blog ou site.

Podemos habilitar a exibição de um campo de entrada extra para o URL do site do usuário definindo a flag **enableCommenterLinks** como true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Quando esse URL for fornecido, a conta do usuário será atualizada e o nome do usuário em todos os comentários, passados e futuros, será vinculado a esse URL.

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]