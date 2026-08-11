[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments não exibe uma lista de usuários na página.

Você pode renderizar uma lista de pessoas que estão visualizando a página no momento, ao lado do widget de comentários. A lista é atualizada em tempo real à medida que os usuários entram e saem, e mostra seu nome, avatar e um indicador online.

Existem três opções de layout:

- `1` - Top: uma linha horizontal de avatares sobrepostos renderizada acima dos comentários.
- `2` - Left: uma barra lateral com nomes e pontos online renderizada à esquerda do widget.
- `3` - Right: a mesma barra lateral renderizada à direita do widget.

Defina a flag **usersListLocation** para habilitar o recurso:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Por padrão, a lista mostra apenas os usuários atualmente online. Para também incluir pessoas que comentaram na página no passado (mas que não estão visualizando no momento), defina **usersListIncludeOffline** como true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Os comentaristas passados são renderizados sem o ponto verde online, para que fique claro quem está presente agora.

Usuários com perfis privados são exibidos com um avatar genérico e um rótulo "Perfil Privado" para que a contagem permaneça precisa sem revelar identidades.

Isso também pode ser configurado sem código. Na página de personalização do widget, veja a opção "Users List Location". Quando a localização é definida para algo diferente de Off, uma caixa de seleção "Include past commenters" aparece abaixo dela.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Localização da Lista de Usuários definida como Direita, com a caixa de seleção incluir comentaristas passados mostrada abaixo'; title='Configurações da Lista de Usuários'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Com base nos últimos 500 usuários ativos, a lista pode estar desatualizada em até 30 segundos.