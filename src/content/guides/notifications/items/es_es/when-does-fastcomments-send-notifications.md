En FastComments, sabemos que ya reciben suficientes notificaciones. Por ello, tomamos medidas para limitar las notificaciones que reciben los usuarios mientras los mantenemos en contacto con sus comunidades. También queremos mantener a los administradores y moderadores al día y avisarles cuando se deba tomar alguna medida.

#### Enviaremos notificaciones para los siguientes eventos para administradores y moderadores:

- Resumen digest de la comunidad (frecuencia configurable).
- Solicitudes de ayuda y recordatorios de la comunidad.
- Comentarios nuevos.

#### Para los comentaristas:

- Cuando alguien responda a su comentario (vía correo electrónico).
- Cuando se le mencione (notificación en la aplicación y por correo electrónico).
- Cuando alguien responda en el mismo hilo (notificación en la aplicación y por correo electrónico).
- Cuando alguien responda a un comentario hijo en el mismo hilo (notificación en la aplicación y por correo electrónico).
- Cuando alguien responda en una página a la que se ha suscrito (notificación en la aplicación y por correo electrónico, frecuencia configurable por suscripción: cada minuto, cada hora o diariamente).
- Cuando un usuario comente por primera vez (pero no con SSO).
- Cuando un usuario deja un comentario en una sesión que no está verificada (pero no con SSO).
  - No enviamos múltiples correos de verificación en este caso. Solo el primero, que verificará toda la actividad en la misma sesión.

#### Para todos los usuarios:

- Cuando se detecta un inicio de sesión desde una nueva dirección IP, se envía un correo de alerta de seguridad con la ubicación aproximada y la dirección IP. Esto no se aplica al primer inicio de sesión del usuario.

#### ...y finalmente solo para administradores:

- Cuando las integraciones se completan.
- Cuando las migraciones se completan.
- Cuando las importaciones o exportaciones finalizan.
- Cuando hay problemas de facturación.
- Recordatorios de fin de prueba.

Algunas notificaciones se agrupan para evitar el envío masivo de notificaciones a los usuarios. Más información al respecto en la siguiente sección `Notification Types`.