---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una campana de notificaciones en la esquina superior derecha del área de comentarios.

Esta campana se pondrá roja y mostrará un recuento del número de notificaciones que tiene el usuario. Algunos ejemplos de notificaciones son:

- Un usuario te respondió.
- Un usuario respondió en un hilo en el que comentaste.
- Un usuario votó a favor de tu comentario.
- Un usuario respondió en una página a la que estás suscrito.

La campana de notificaciones también ofrece un mecanismo para suscribirse a una página completa.

Sin embargo, podemos desactivar la campana de notificaciones por completo:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Esto también puede hacerse sin código. En la página de personalización del widget, consulta la sección "Disable Notification Bell".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---