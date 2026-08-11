[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una campana de notificaciones en la esquina superior derecha del área de comentarios.

Esta campana se volverá roja y mostrará un recuento del número de notificaciones que tiene el usuario. Algunas notificaciones de ejemplo son:

- El usuario te respondió.
- El usuario respondió en un hilo en el que comentaste.
- El usuario votó positivamente tu comentario.
- El usuario respondió a una página a la que te has suscrito.

La campana de notificaciones también proporciona un mecanismo para suscribirse a una página completa.

Sin embargo, podemos desactivar la campana de notificaciones por completo:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Desactivar la campana de notificaciones'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la sección "Desactivar la campana de notificaciones".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Página de personalización del widget con la casilla Desactivar la campana de notificaciones marcada'; title='Desactivar la campana de notificaciones' app-screenshot-end]