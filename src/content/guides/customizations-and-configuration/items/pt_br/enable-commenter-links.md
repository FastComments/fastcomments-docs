[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments solicitará ao usuário apenas seu comentário, seu nome de usuário e seu e‑mail.

No entanto, em algumas situações você pode querer que o usuário deixe um link para seu próprio blog ou site.

Podemos habilitar a exibição de um campo de entrada extra para inserir a URL do site do usuário definindo a flag **enableCommenterLinks** como true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Habilitando Links de Comentadores'; code-example-end]

Quando essa URL for fornecida, a conta do usuário será atualizada e todos os seus nomes de usuário em comentários passados e futuros serão vinculados a essa URL.

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Página de personalização do widget com a caixa de seleção de links do comentador marcada para adicionar um campo de URL de site ao formulário de comentário'; title='Habilitando Links de Comentadores' app-screenshot-end]