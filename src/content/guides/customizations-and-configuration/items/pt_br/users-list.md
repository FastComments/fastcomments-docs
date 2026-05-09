[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não exibe uma lista de usuários na página.

Você pode renderizar uma lista de pessoas que estão atualmente visualizando a página, ao lado do widget de comentários. A lista é atualizada em tempo real à medida que os usuários entram e saem, e mostra o nome, avatar e um indicador de online.

Há três opções de layout:

- `1` - Topo: uma linha horizontal de avatares sobrepostos renderizada acima dos comentários.
- `2` - Esquerda: uma barra lateral com nomes e pontos de online renderizada à esquerda do widget.
- `3` - Direita: a mesma barra lateral renderizada à direita do widget.

Defina a flag **usersListLocation** para habilitar o recurso:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por padrão a lista mostra apenas usuários atualmente online. Para também incluir pessoas que comentaram na página no passado (mas que não estão visualizando-a no momento), defina **usersListIncludeOffline** como true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Usuários que comentaram anteriormente são exibidos sem o ponto verde de online, para que fique claro quem está presente agora.

Usuários com perfis privados são mostrados com um avatar genérico e um rótulo "Perfil Privado", assim a contagem permanece precisa sem revelar identidades.

Isto também pode ser configurado sem código. Na página de personalização do widget, veja a opção "Localização da Lista de Usuários". Quando a localização estiver definida para qualquer valor diferente de Desativado, uma caixa de seleção "Incluir comentadores anteriores" aparece abaixo dela.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Para mais de 500 usuários ao vivo, a lista pode ficar até 30 segundos desatualizada.