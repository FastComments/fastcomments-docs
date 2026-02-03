FastComments te permite exigir que los comentaristas primerizos acepten tus Términos de Servicio antes de enviar un comentario.

Cuando está habilitado:
- **Usuarios anónimos** verán una casilla de TOS cada vez que comenten
- **Usuarios autenticados** verán la casilla solo en su primer comentario, o cuando actualices tus TOS

### Configuration

Navega a la página de personalización del widget y habilita la casilla "Require Terms of Service acceptance". Una vez habilitada, verás las siguientes opciones:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Por defecto, la casilla muestra "I agree to the Terms of Service and Privacy Policy" con enlaces a ambos documentos. Selecciona "Customize text per locale" para proporcionar tu propio texto para cada idioma.
- **TOS Last Updated Date**: Cuando actualices tus Términos de Servicio, establece esta fecha. Los usuarios que aceptaron antes de esta fecha deberán aceptar de nuevo.

### How It Works

- La marca de tiempo de aceptación de los TOS se almacena por usuario y por comentario
- Cuando un usuario acepta los TOS, la fecha se registra en su perfil de usuario (per-tenant)
- Si estableces una fecha "Last Updated" que sea posterior a la fecha de aceptación del usuario, tendrá que aceptar de nuevo
- Para los usuarios anónimos que no pueden ser rastreados, la casilla aparece en cada envío de comentario

---