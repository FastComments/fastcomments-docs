---
Por defecto, cada usuario puede enviar hasta `5 comentarios` en el mismo minuto.

Esto se rastrea por ID de usuario, ID de usuario anónimo y dirección IP (hasheada).

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Campo de máximo de comentarios por minuto en la página de personalización del widget, configurado a 5 por defecto'; title='Limitando el volumen de comentarios por usuario' app-screenshot-end]

Ten en cuenta que si estás usando la API de creación de comentarios, puede que quieras pasar la dirección `ip` original del usuario en la solicitud a nuestro backend para que la limitación de velocidad se aplique por usuario y no globalmente a tu cuenta.

---