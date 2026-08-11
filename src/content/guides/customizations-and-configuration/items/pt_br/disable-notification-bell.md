[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments exibirá um sino de notificação no canto superior direito da área de comentários.

Esse sino ficará vermelho e mostrará a contagem de notificações que o usuário tem. Alguns exemplos de notificações são:

- Usuário respondeu a você.
- Usuário respondeu em um tópico no qual você comentou.
- Usuário deu voto positivo ao seu comentário.
- Usuário respondeu a uma página à qual você está inscrito.

O sino de notificação também fornece um mecanismo para assinar uma página inteira, como também.

No entanto, podemos desativar o sino de notificação completamente:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Desativar Sino de Notificação'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Desativar Sino de Notificação".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Página de personalização do widget com a caixa de seleção Desativar Sino de Notificação marcada'; title='Desativar Sino de Notificação' app-screenshot-end]