[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Por padrão, as respostas aos comentários de nível superior são exibidas.

Isso pode ser configurado para que o usuário precise clicar em "Mostrar respostas" nos comentários de nível superior para ver as respostas.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Recolher respostas aos comentários de nível superior'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opção de recolher respostas na interface de personalização do widget, ocultando comentários filhos atrás de um link Mostrar respostas'; title='Recolher respostas' app-screenshot-end]

Esta configuração não afetará o número de comentários de nível superior carregados inicialmente. Se você tem um comentário de nível superior e 29 respostas, com esta configuração ativada você irá:

- Ver o comentário de nível superior.
- Ver Mostrar respostas (29) sob este comentário.

Se você deseja mostrar todos os comentários de nível superior em combinação com esta opção, defina [página inicial para -1](#starting-page).