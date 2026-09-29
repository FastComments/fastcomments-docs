Cuando los usuarios comentan o votan y no han iniciado sesión, se les pedirá que proporcionen su correo electrónico y nombre de usuario.

En algunos sitios, pedir a un visitante que invente un nombre de usuario único es un obstáculo, particularmente en dispositivos móviles. FastComments puede generar un nombre de usuario neutral para cada nuevo visitante y completarlo automáticamente en el campo de nombre de usuario, como `BraveOtter4172`.

El visitante puede dejarlo tal cual, o reemplazarlo con un nombre de su elección.

Esto se puede habilitar desde la interfaz de personalización, bajo la configuración llamada `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='La opción Generar nombres de usuario automáticamente en la interfaz de personalización del widget'; title='Generar nombres de usuario automáticamente' app-screenshot-end]

#### Cómo funciona

- Cada nombre generado es único. Se verifica contra las cuentas existentes y se reserva para la sesión del navegador de ese visitante, de modo que a dos visitantes no se les ofrezca el mismo nombre.
- El nombre solo se genera para los visitantes que aún no tienen uno. Los usuarios que han iniciado sesión, los usuarios SSO y los visitantes que ya han comentado conservan su nombre existente.
- Funciona con o sin [comentario anónimo](/guide-customizations-and-configuration.html#allow-anon). Con el comentario anónimo desactivado, el visitante sigue ingresando su correo electrónico, pero ya no tiene que pensar en un nombre de usuario.
- Un visitante que regresa y escribe un correo electrónico que ya ha usado se asocia a su cuenta existente y conserva el nombre en esa cuenta.
- Si también se ha configurado un [Nombre de usuario predeterminado](/guide-customizations-and-configuration.html#default-username), el nombre generado tiene prioridad.

---