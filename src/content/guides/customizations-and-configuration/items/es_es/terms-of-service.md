---
FastComments le permite exigir a los usuarios que comentan por primera vez que acepten sus Términos del Servicio antes de enviar un comentario.

Cuando está habilitado:
- **Usuarios anónimos** verán una casilla de aceptación de los Términos del Servicio cada vez que comenten
- **Usuarios autenticados** verán la casilla solo en su primer comentario, o cuando usted actualice sus Términos del Servicio

### Habilitar los Términos del Servicio

Vaya a la página de personalización del widget y active la casilla "Require Terms of Service acceptance":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Personalizar el texto de los Términos del Servicio

Por defecto, la casilla muestra "I agree to the Terms of Service and Privacy Policy" con enlaces a ambos documentos. Puede personalizar este texto por idioma si lo desea:

1. Seleccione "Customize text per locale"
2. Seleccione el idioma desde el menú desplegable e introduzca su texto personalizado

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Actualizar sus Términos del Servicio

Cuando actualice sus Términos del Servicio, establezca la fecha "Last Updated". Los usuarios que aceptaron los Términos del Servicio antes de esa fecha tendrán que aceptar de nuevo:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Cómo funciona

- La marca de tiempo de aceptación de los Términos del Servicio se almacena por usuario y por comentario
- Cuando un usuario acepta los Términos del Servicio, la fecha se registra en su perfil de usuario (per-tenant)
- Si establece una fecha "Last Updated" que sea posterior a la fecha de aceptación del usuario, tendrán que aceptar de nuevo
- Para los usuarios anónimos que no pueden ser rastreados, la casilla aparece en cada envío de comentario

---