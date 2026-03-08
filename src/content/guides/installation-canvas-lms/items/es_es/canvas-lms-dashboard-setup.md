#### Navegar a la Configuración LTI de Canvas

Inicia sesión en tu cuenta de FastComments y ve a <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Mi cuenta > Configuración LTI de Canvas</a>.

#### Crear una nueva configuración LTI

Marca la casilla **Habilitar LTI**. Aparecerán los campos de configuración:

- **Configuration Name** - una etiqueta opcional para identificar esta configuración (útil si conectas múltiples instancias de Canvas).
- **Platform URL** - la URL de tu instancia de Canvas (p. ej. `https://yourschool.instructure.com`). Puedes dejar esto en blanco por ahora y completarlo después de crear la Developer Key.
- **Client ID** - déjalo en blanco por ahora. Lo rellenarás después de crear la Developer Key en Canvas.
- **Deployment ID** - déjalo en blanco por ahora.
- **Comment Style** - elige entre Comments, Collab Chat o Both. Consulta [Estilos de comentario](#canvas-lms-commenting-styles) para más detalles.

Haz clic en **Agregar** para crear la configuración.

#### Copiar las URLs de configuración

Después de guardar, aparecerán tres URLs:

- **Configuration URL** - pegarás esto en Canvas al crear la Developer Key.
- **OIDC Login URL** - usado por Canvas para el flujo de inicio de sesión LTI (configurado automáticamente vía la Configuration URL).
- **Launch URL** - el endpoint que Canvas llama cuando un estudiante abre FastComments (configurado automáticamente vía la Configuration URL).

Copia la **Configuration URL**. La necesitarás en el siguiente paso.