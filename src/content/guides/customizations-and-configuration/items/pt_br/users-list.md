[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não mostra uma lista de usuários na página.

Você pode renderizar uma lista de pessoas que estão visualizando a página no momento, junto ao widget de comentários. A lista é atualizada em tempo real à medida que os usuários entram e saem, e mostra o nome, avatar e um indicador de online.

Existem três opções de layout:

- `1` - Top: uma linha horizontal de avatares sobrepostos renderizada acima dos comentários.
- `2` - Left: uma barra lateral com nomes e pontos de online renderizada à esquerda do widget.
- `3` - Right: a mesma barra lateral renderizada à direita do widget.

Defina a flag **usersListLocation** para habilitar o recurso:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por padrão, a lista mostra apenas os usuários atualmente online. Para também incluir pessoas que comentaram na página no passado (mas que não estão visualizando agora), defina **usersListIncludeOffline** como true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Comentadores anteriores são exibidos sem o ponto verde de online para que fique claro quem está presente agora.

Usuários com perfis privados são mostrados com um avatar genérico e um rótulo "Perfil Privado" para que a contagem permaneça precisa sem revelar identidades.

Isto também pode ser configurado sem código. Na página de customização do widget, veja a opção "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Quando a localização estiver definida para qualquer opção diferente de Off, a caixa de seleção "Include past commenters" é exibida logo abaixo:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]