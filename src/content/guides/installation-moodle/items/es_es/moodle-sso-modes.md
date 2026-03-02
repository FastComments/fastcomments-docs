---
El plugin admite tres modos de SSO para integrar las cuentas de usuario de Moodle con FastComments.

#### SSO seguro (recomendado)

Los datos de usuario se firman del lado del servidor usando HMAC-SHA256 con su API Secret. Esta es la opción más segura y se recomienda para uso en producción.

Con SSO seguro:

- Los nombres de usuario, correos electrónicos y avatares se envían de forma segura a FastComments.
- Los administradores del sitio de Moodle se sincronizan automáticamente como moderadores de FastComments.
- Los usuarios no pueden suplantar otras cuentas.
- Requiere que el **API Secret** esté configurado en la configuración del plugin.

#### SSO simple

Los datos de usuario (nombre, correo electrónico, avatar) se envían desde el cliente sin una firma criptográfica. Esto es más fácil de configurar ya que no requiere un API Secret, pero es menos seguro porque los datos de usuario no se verifican del lado del servidor.

#### Ninguno

Sin integración SSO. Los usuarios comentan de forma anónima o deben iniciar sesión en FastComments por separado. Use esto si no desea que las cuentas de usuario de Moodle estén vinculadas a los comentarios.

---