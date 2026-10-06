---
En FastComments, sabemos que ya recibes suficientes notificaciones. Por eso, tomamos algunas medidas para limitar las notificaciones que los usuarios reciben, sin dejar de mantenerlos en contacto con sus comunidades. También queremos mantener a los administradores y moderadores al día y avisarles cuando sea necesario tomar alguna acción.

#### Enviaremos notificaciones para los siguientes eventos para administradores y moderadores:

- Resumen del Digest de la Comunidad (frecuencia configurable).
- Solicitudes de ayuda de la comunidad y recordatorios.
- Nuevos comentarios.

#### Para los comentaristas:

- Cuando alguien responde a tu comentario (por correo electrónico).
- Cuando eres mencionado (notificación en la aplicación y por correo electrónico).
- Cuando alguien responde en el mismo hilo (notificación en la aplicación y por correo electrónico).
- Cuando alguien responde a un comentario hijo en el mismo hilo (notificación en la aplicación y por correo electrónico).
- Cuando alguien responde a una página a la que te has suscrito (notificación en la aplicación y por correo electrónico, frecuencia configurable por suscripción: cada minuto, cada hora o diariamente).
- Cuando un usuario comenta por primera vez (pero no con SSO).
- Cuando un usuario deja un comentario en una sesión que no está verificada (pero no con SSO).
  - No enviamos varios correos de verificación en este caso. Solo el primero, que verificará toda la actividad en la misma sesión.

#### Para todos los usuarios:

- Cuando se detecta un inicio de sesión desde una nueva dirección IP, se envía un correo de alerta de seguridad con la ubicación aproximada y la dirección IP. Esto no se aplica al primer inicio de sesión del usuario.

#### ...y finalmente solo para administradores:

- Cuando las integraciones están completas.
- Cuando las migraciones están completas.
- Cuando las importaciones o exportaciones finalizan.
- Cuando hay problemas de facturación.
- Recordatorios de fin de prueba.

Algunas notificaciones se agrupan para evitar el envío masivo de notificaciones a los usuarios. Aprende sobre esto en la siguiente sección `Notification Types`.

Los correos de respuesta y mención solo se envían para los comentarios que están aprobados. Para ver si se envió un correo de respuesta o mención para un comentario específico, o por qué no se envió, abre los [Registros de Comentarios](/guide-moderation.html#comment-logs) de ese comentario desde la página Moderar Comentarios.

---