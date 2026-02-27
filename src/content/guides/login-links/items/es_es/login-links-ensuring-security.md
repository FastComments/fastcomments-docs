Dado que los enlaces de inicio de sesión son esencialmente contraseñas, nos tomamos la seguridad muy en serio.

Todos los enlaces de inicio de sesión en nuestro sistema están configurados para expirar después de un cierto período de tiempo, y también tenemos mecanismos para detectar
la adivinación de un enlace de inicio de sesión. Algunos enlaces de inicio de sesión se dividen en múltiples contraseñas, y si se adivina una,
la otra será invalidada.

### Seguridad en comparación con las contraseñas

Con la mayoría de los sistemas que requieren una contraseña, puedes pasar por un mecanismo de Olvidé mi contraseña
si tienes el correo electrónico del usuario. Esto significa que, si tienes acceso a la cuenta de correo electrónico del usuario,
no importa si el sistema atacado utiliza contraseñas o enlaces mágicos.

### Alertas de inicio de sesión desde una IP nueva

Cuando se produce un inicio de sesión desde una dirección IP que no se ha visto antes para una cuenta dada, FastComments envía un correo electrónico de alerta de seguridad
con la ubicación aproximada y la dirección IP. Esto ayuda a los usuarios a detectar accesos no autorizados. Tenga en cuenta que FastComments no almacena
direcciones IP sin procesar — solo se guarda una forma ofuscada por motivos de seguridad.

### Correo electrónico de respaldo para la recuperación de la cuenta

Si pierdes el acceso a tu correo electrónico principal, puedes usar un correo electrónico de respaldo verificado para recuperar tu cuenta. Tu correo de respaldo funciona
con todos los flujos de inicio de sesión. Puedes ingresarlo en la página de olvidé mi nombre de usuario, usarlo con el inicio de sesión por enlace mágico, o escribirlo en el
campo de nombre de usuario/correo electrónico para el inicio de sesión con contraseña.

Para configurar un correo de respaldo, ve a [Detalles de la cuenta](https://fastcomments.com/auth/my-account/edit-details) y haz clic
**Definir un correo electrónico de respaldo**. Tu correo de respaldo se usa únicamente para la recuperación de la cuenta y no recibirá notificaciones.

### Seguridad en comparación con MFA

Los enlaces de inicio de sesión son menos seguros que MFA. FastComments ahora admite la autenticación de dos factores (2FA)
para las cuentas de administrador para proporcionar mayor seguridad. Cuando 2FA está habilitado, es obligatorio incluso cuando se usan enlaces de inicio de sesión.