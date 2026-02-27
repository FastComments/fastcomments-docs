Dado que los enlaces de inicio de sesión son esencialmente contraseñas, nos tomamos la seguridad muy en serio.

Todos los enlaces de inicio de sesión en nuestro sistema están configurados para expirar después de un cierto período de tiempo, y también tenemos mecanismos para detectar la adivinanza de un enlace de inicio de sesión. Algunos enlaces de inicio de sesión se dividen en varias contraseñas, y si se adivina una, la otra se invalidará.

### Seguridad en comparación con las contraseñas

Con la mayoría de los sistemas que requieren una contraseña, puedes pasar por un mecanismo de Olvidé mi contraseña si tienes el correo electrónico del usuario. Esto significa que, si tienes acceso a la cuenta de correo electrónico del usuario, no importa si el sistema bajo ataque usa contraseñas o enlaces mágicos.

### Alertas de inicio de sesión desde nuevas IP

Cuando un inicio de sesión se produce desde una dirección IP que no se había visto antes para una cuenta determinada, FastComments envía un correo electrónico de alerta de seguridad con la ubicación aproximada y la dirección IP. Esto ayuda a los usuarios a detectar accesos no autorizados. Tenga en cuenta que FastComments no almacena direcciones IP en bruto — solo se almacena una forma ofuscada por motivos de seguridad.

### Seguridad en comparación con MFA

Los enlaces de inicio de sesión son menos seguros que MFA. FastComments ahora admite la autenticación de dos factores (2FA) para cuentas de administrador para proporcionar mayor seguridad. Cuando 2FA está habilitada, es obligatoria incluso al usar enlaces de inicio de sesión.