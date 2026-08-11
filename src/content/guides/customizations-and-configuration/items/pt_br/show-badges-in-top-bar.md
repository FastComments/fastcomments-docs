[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments exibirá os emblemas dos usuários apenas em seus comentários dentro da thread de comentários.

No entanto, podemos mostrar os emblemas dos usuários ao lado de seu nome acima do formulário de comentário habilitando este recurso na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Caixa de seleção Mostrar emblemas na barra superior na página de personalização do widget, posicionando emblemas ao lado do nome acima do formulário de comentário'; title='Opção Mostrar Emblemas na Barra Superior' app-screenshot-end]

Isso exibirá os emblemas do usuário ao lado de seu nome na área da barra superior, tornando suas conquistas e status mais proeminentes enquanto ele está compondo um comentário.

Observe que este recurso deve estar habilitado na interface de personalização do widget para funcionar. Você pode, opcionalmente, definir a flag **showBadgesInTopBar** como false na sua configuração de código para desativá‑lo seletivamente mesmo quando ele está ativado no nível do servidor:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]