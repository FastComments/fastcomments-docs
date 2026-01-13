[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments permite que usuários bloqueiem outros usuários. Bloquear um usuário fará com que seus comentários
sejam mascarados, impedirá notificações entre os usuários e assim por diante.

Pode ser desejável desativar essa funcionalidade. Isso pode ser feito da seguinte forma:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Isso também pode ser feito sem código, o que também habilita a validação adequada no lado do servidor, pela Interface de Personalização do Widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]