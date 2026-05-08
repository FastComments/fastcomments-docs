[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não exibe uma lista de usuários na página.

Você pode exibir uma lista de pessoas que estão visualizando a página, ao lado do widget de comentários. A lista é atualizada em tempo real à medida que os usuários entram e saem, e mostra o nome, o avatar e um indicador de online.

Existem três opções de layout:

- `1` - Topo: uma linha horizontal de avatares sobrepostos renderizados acima dos comentários.
- `2` - Esquerda: uma barra lateral com nomes e indicadores online renderizada à esquerda do widget.
- `3` - Direita: a mesma barra lateral renderizada à direita do widget.

Defina a flag **usersListLocation** para ativar o recurso:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por padrão, a lista mostra apenas os usuários atualmente online. Para também incluir pessoas que comentaram na página no passado (mas que não a estão visualizando agora), defina **usersListIncludeOffline** como true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Os comentaristas anteriores são exibidos sem o ponto verde de online, assim fica claro quem está presente no momento.

Usuários com perfis privados são exibidos com um avatar genérico e um rótulo "Perfil Privado" para que a contagem permaneça precisa sem revelar identidades.

Isso também pode ser configurado sem código. Na página de personalização do widget, veja a opção "Localização da lista de usuários". Quando a localização for definida para qualquer opção diferente de "Desativado", uma caixa de seleção "Incluir comentaristas anteriores" aparece abaixo dela.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---