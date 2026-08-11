[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments permite que os usuários bloqueiem outros usuários. Bloquear um usuário fará com que seus comentários sejam mascarados, impede notificações entre os usuários, e assim por diante.

Pode ser desejável desativar essa funcionalidade. Isso pode ser feito da seguinte maneira:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Isso também pode ser feito sem código, o que também habilita a validação adequada no lado do servidor, via a interface de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opção de desativar bloqueio na interface de personalização do widget, que impede que os usuários bloqueiem uns aos outros'; title='Desativar bloqueio' app-screenshot-end]