FastComments le permite exigir que los comentaristas que comentan por primera vez acepten sus Términos de Servicio antes de enviar un comentario.

Cuando está habilitado:
- **Usuarios anónimos** verán una casilla de verificación de TOS cada vez que comenten
- **Usuarios autenticados** verán la casilla solo en su primer comentario, o cuando actualice sus TOS

### Configuración

Navegue a la página de personalización del widget y habilite la casilla "Require Terms of Service acceptance". Una vez habilitada, verá las siguientes opciones:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel de Términos de Servicio que muestra el selector de modo de texto del TOS y el campo de fecha de última actualización'; title='Opciones de Términos de Servicio' app-screenshot-end]

- **Modo de texto del TOS**: Por defecto, la casilla muestra "Acepto los Términos de Servicio y la Política de Privacidad" con enlaces a ambos documentos. Seleccione "Personalizar texto por idioma" para proporcionar su propio texto para cada idioma.
- **Fecha de última actualización del TOS**: Cuando actualice sus Términos de Servicio, establezca esta fecha. Los usuarios que aceptaron antes de esta fecha deberán volver a aceptar.

### Cómo funciona

- La marca de tiempo de aceptación del TOS se almacena por usuario y por comentario
- Cuando un usuario acepta el TOS, la fecha se registra en su perfil de usuario (por inquilino)
- Si establece una fecha de "Última actualización" que sea posterior a la fecha de aceptación del usuario, necesitarán volver a aceptar
- Para usuarios anónimos que no pueden ser rastreados, la casilla aparece en cada envío de comentario