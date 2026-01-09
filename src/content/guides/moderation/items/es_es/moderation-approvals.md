---
Con FastComments, un comentario Aprobado es un comentario **visible**. Para ocultar un comentario, lo desaprobarías.

Los comentarios pueden ser aprobados automáticamente, o los moderadores pueden aprobar manualmente cada comentario. También existe la opción de
solo requerir la aprobación del primer comentario de un usuario - en cuyo caso los comentarios posteriores se aprueban automáticamente y no requieren moderación.

FastComments tiene el concepto de Verificado vs No verificado. Los comentarios verificados se publican ya sea en una sesión verificada por correo electrónico (el usuario está completamente conectado, o usa SSO)
o se publicaron en un estado No verificado y luego fueron verificados manualmente por correo electrónico.

La noción de Verificado puede ocultarse completamente si se desea mediante reglas de personalización.

Requerir la aprobación manual de comentarios no verificados puede ayudar con el spam, ya que los bots rara vez verifican sus comentarios por correo electrónico. En este caso
debería habilitar la aprobación automática de comentarios pero activar `Only Auto Approve Verified Comments`.

Esto puede configurarse todo por los administradores desde [Configuración de moderación](https://fastcomments.com/auth/my-account/moderate-comments/settings).

---