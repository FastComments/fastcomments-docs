[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Por padrão, as respostas aos comentários de nível superior são exibidas.

Isso pode ser configurado para que o usuário precise clicar em "Mostrar respostas" nos comentários de nível superior para ver os comentários filhos.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Essa configuração não afetará o número de comentários de nível superior carregados inicialmente. Se você tiver um comentário de nível superior e 29 comentários filhos, com essa configuração ativada você irá:

- Ver o comentário de nível superior.
- Ver 'Mostrar respostas (29)' abaixo deste comentário.

Se você desejar exibir todos os comentários de nível superior em combinação com esta opção, defina [a página inicial para -1](#starting-page).