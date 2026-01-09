Por defecto, cada usuario puede enviar hasta `5 comments` en el mismo minuto.

Esto se rastrea por user id, anon user id y ip address (hashed).

Esto puede personalizarse sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Ten en cuenta que si estás usando la comment creation API puede que quieras pasar la dirección `ip` original del usuario en la solicitud a nuestro backend para que la limitación de tasa se aplique
por usuario y no de forma global a tu cuenta.