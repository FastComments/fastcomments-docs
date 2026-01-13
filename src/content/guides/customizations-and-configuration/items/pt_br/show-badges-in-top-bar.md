---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments exibirá os distintivos dos usuários apenas em seus comentários dentro do thread de comentários.

No entanto, podemos mostrar os distintivos dos usuários ao lado do nome deles acima do formulário de comentário ativando esse recurso na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Isso exibirá os distintivos do usuário ao lado do nome na área da barra superior, tornando suas conquistas e status mais evidentes enquanto ele está compondo um comentário.

Observe que esse recurso precisa ser habilitado na interface de personalização do widget para funcionar. Você pode opcionalmente definir a flag **showBadgesInTopBar** como false na sua configuração de código para desativá‑lo seletivamente mesmo quando estiver ativado no nível do servidor:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---