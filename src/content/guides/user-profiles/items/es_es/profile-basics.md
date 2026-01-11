Los perfiles de usuario en FastComments proporcionan un espacio dedicado para que cada usuario muestre su identidad, actividad y contribuciones en toda tu comunidad.

### ¿Qué es un perfil de usuario?

Un perfil de usuario es una página personalizada para cada usuario de FastComments que muestra:

- **Profile Header** - Una imagen de fondo personalizable que personaliza el perfil
- **Avatar** - La foto de perfil del usuario con indicador de estado en línea/fuera de línea
- **Display Information** - Nombre de usuario, nombre para mostrar y bandera de país opcional
- **Bio** - Una descripción personal o introducción
- **Social Links** - Conexiones a los perfiles de redes sociales y al sitio web del usuario
- **Badges** - Insignias, logros y reconocimientos obtenidos
- **Statistics** - Karma del usuario y número total de comentarios
- **Communities** - En qué sitios/dominios está activo el usuario

### Acceso a los perfiles de usuario

Hay varias maneras de acceder al perfil de un usuario:

1. **Click on an avatar** - En el widget de comentarios, haz clic en el avatar de cualquier usuario para ver su perfil
2. **Click on a username** - Los nombres de usuario en los comentarios son enlaces clicables a los perfiles
3. **Direct URL** - Visita `https://fastcomments.com/auth/user-profile/[userId]`

### Vistas del perfil

Al ver un perfil, verás diferentes pestañas dependiendo de si estás viendo tu propio perfil o el de otra persona:

#### Tu propio perfil
- **Notifications** - Tus notificaciones y menciones
- **Recent Activity** - Tu historial de comentarios en todas las comunidades
- **Profile Comments** - Comentarios que otras personas han dejado en la página de tu perfil
- **Direct Messages** - Conversaciones privadas con otros usuarios

#### Perfiles de otros usuarios
- **Recent Activity** - Su historial público de comentarios (si no lo tienen configurado como privado)
- **Profile Comments** - Comentarios en su perfil (si no lo tienen configurado como privado)
- **Direct Messages** - Iniciar o continuar una conversación privada (si permiten mensajes directos)

### Estado en línea

Los perfiles de usuario muestran el estado en línea en tiempo real:
- **Green indicator** - El usuario está en línea actualmente
- **No indicator** - El usuario está desconectado

Esto te ayuda a saber cuándo alguien está usando activamente la plataforma, lo cual es especialmente útil para los mensajes directos.

### Tipos de usuario

FastComments admite dos tipos de usuarios con perfiles:

1. **Regular Users** - Usuarios que se registraron directamente en FastComments
2. **SSO Users** - Usuarios que se autentican a través de la integración de inicio de sesión único (SSO) de tu sitio

Ambos tipos de usuarios tienen acceso al sistema completo de perfiles, aunque los usuarios SSO pueden tener algunas restricciones para editar ciertos campos (como los avatares) según la configuración de tu SSO.