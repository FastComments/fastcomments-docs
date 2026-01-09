[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments exibirá um sino de notificações no canto superior direito da área de comentários.

Esse sino ficará vermelho e mostrará uma contagem do número de notificações que o usuário possui. Alguns exemplos de notificações são:

- Um usuário respondeu para você.
- Um usuário respondeu em um tópico no qual você comentou.
- Um usuário votou positivamente no seu comentário.
- Um usuário respondeu em uma página à qual você está inscrito.

O sino de notificações também fornece um mecanismo para se inscrever em uma página inteira.

No entanto, podemos desativar o sino de notificações inteiramente:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Desativar sino de notificações".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]