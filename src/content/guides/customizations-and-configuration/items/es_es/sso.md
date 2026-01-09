SSO, o inicio de sesión único, es un conjunto de convenciones que se usan para permitir que tú o tus usuarios utilicen FastComments sin tener que crear otra cuenta.

Suponiendo que no permites comentarios anónimos, se requiere una cuenta para comentar con FastComments. Hacemos que este proceso de registro sea muy sencillo: el usuario simplemente deja su correo electrónico cuando comenta.
Sin embargo, entendemos que incluso eso es una fricción adicional que algunos sitios quieren evitar.

Podemos reducir esa fricción teniendo solo un flujo de inicio de sesión para todo tu sitio.

### ¿Cómo lo obtengo?
Todos los tipos de cuenta actualmente obtienen acceso a SSO. Sin embargo, el número máximo de usuarios SSO variará dependiendo de tu paquete. Como con otras funcionalidades, los planes Pro y superiores proporcionan soporte de desarrollo directo.

Comparemos las opciones, y luego entremos en los detalles de cada una.

### Migraciones de Usuarios y Comentarios

Al migrar desde una plataforma con SSO como Disqus, ya tendrás usuarios y sus comentarios.

Los comentarios se importan como parte de tu migración, ya sea mediante la API, nuestra interfaz de importación, o el soporte al cliente. La interfaz de importación es preferida si soporta la plataforma desde la que
estás migrando, ya que incorpora manejo de errores, extracción y subida de avatar y medios, y un sistema de monitoreo por lotes.

Los propios usuarios se agregan automáticamente al ver los hilos de comentarios por primera vez. Alternativamente, pueden pre-agregarse mediante la API, pero este trabajo no tiene
muchas ventajas.

Si los comentarios se importan, y los usuarios SSO no se agregan manualmente vía la API, entonces los comentarios se migrarán automáticamente a la cuenta del usuario la primera
vez que se cree cuando vean cualquier hilo de comentarios. Entonces podrán gestionar, editar y eliminar los comentarios que escribieron originalmente.

La migración automática se realiza mediante correo electrónico o nombre de usuario. Algunas plataformas no proporcionan correos electrónicos en la exportación, como Disqus, por lo que en ese caso recurrimos al nombre de usuario.
- Siempre que pases un nombre de usuario coincidente, y un correo electrónico en el payload SSO, añadiremos el correo electrónico a los objetos de comentario individuales para que las notificaciones y menciones funcionen.

Si se desea importar tus comentarios y usuarios a la vez, trabaja con soporte para migrar los comentarios a las respectivas cuentas de los usuarios después de que los usuarios sean importados
vía la API.

Entonces, para resumir, el camino más fácil para la migración es:

1. Importar comentarios.
   1. Los avatares y otros medios se migran automáticamente si usas la Interfaz de Importación en `Manage Data -> Imports`.
2. Configurar SSO Seguro o Simple.
3. Dejar que la migración ocurra por usuario automáticamente cuando inicien sesión por primera vez.
   1. Esto usualmente añade menos de un segundo al tiempo de carga de la página si el usuario tiene menos de 50k comentarios.

### Usuarios de WordPress
Si estás usando nuestro <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin de WordPress</a> ¡no hay código que escribir! Simplemente ve a la página de administración del plugin, haz clic en Configuración SSO, y luego Habilitar.

Esto te llevará a un asistente de un solo botón que creará tu clave API, la enviará a tu instalación de WordPress y activará SSO. Hemos consolidado esto en un solo clic para ti.

Nota que si estás instalando el plugin por primera vez tendrás que completar el proceso de configuración antes de ver la página de administración con el botón de Configuración SSO.

#### SSO de WordPress - Moderadores

Ten en cuenta que actualmente para que la insignia "Moderator" se muestre junto a tus moderadores cuando comentan con el plugin de FastComments para WordPress,
también deben ser añadidos como Moderador en el panel de FastComments, y tener su correo electrónico verificado.

### Integraciones Personalizadas

Para integraciones personalizadas, hay dos opciones.

### Opción Uno - SSO Seguro

Con SSO Seguro, FastComments sabe que el usuario que comenta, vota y lee comentarios es un usuario real de tu sitio.

Siempre que crees un payload válido, el usuario tendrá siempre una experiencia de comentarios sin fricciones.

Con SSO Seguro, el payload SSO se crea **del lado del servidor** usando autenticación HMAC y luego se pasa al widget en el **cliente**.

Con SSO Seguro, la cuenta del usuario está **completamente separada** del resto de la base de usuarios de FastComments. Esto significa que si tenemos dos socios
Company A y Company B, cada uno puede tener un usuario SSO con el nombre de usuario "Bob".

#### Requisitos
- Algunos conocimientos básicos sobre desarrollo backend.
- Algunos conocimientos básicos sobre el manejo de claves API secretas.
- Algunos conocimientos básicos sobre desarrollo de API o renderizado del lado del servidor.

#### Ventajas
- Seguro.
- Experiencia de comentarios sin fricciones.

#### Desventajas
- Requiere desarrollo backend.

#### Actualización de Datos de Usuario

Con SSO Seguro, cada vez que pases el sso user payload, actualizaremos su usuario con la información más reciente. Por ejemplo, si
el usuario tiene un nombre de usuario `X`, y pasas `Y` en el payload SSO, su nombre de usuario se convertirá en `Y`.

Si quieres eliminar valores usando este enfoque entonces configúralos a `null` (no `undefined`).

#### API de SSO Seguro

También proporcionamos una API para interactuar con los usuarios SSO. Ver [la documentación](/guide-api.html#sso-user-structure).

Nota que al usar SSO Seguro, los usuarios se crean automáticamente detrás de escena al cargar la página. No tienes que importar tus usuarios en bloque.

### Opción Dos - SSO Simple

La alternativa al SSO Seguro es simplemente pasar la información del usuario al widget de comentarios.

Proporcionar un correo electrónico con SSO Simple no es obligatorio, sin embargo sin esto sus comentarios aparecerán como "No verificado".

<sup>¡Nota!</sup> A principios de 2022 los nombres de usuario con SSO Simple no necesitan ser únicos en todo FastComments.com.

Idealmente, SSO Simple solo debería elegirse cuando se desarrolla en una plataforma que no proporciona acceso al backend.

#### Requisitos
- Algunos conocimientos básicos sobre desarrollo del lado del cliente.
- Tener al menos el correo electrónico del usuario.

#### Ventajas
- Simple.
- Toda la actividad sigue verificándose.
- El usuario nunca introduce su nombre de usuario o correo electrónico.

#### Desventajas
- Menos seguro que SSO Seguro ya que el payload del lado del cliente podría forjarse para convertirse en cualquier usuario.

#### API de SSO Simple

Los usuarios creados automáticamente a través del flujo SSO Simple se almacenan como objetos `SSOUser`. Pueden accederse y gestionarse vía la API `SSOUser`. Ver [la documentación](/guide-api.html#sso-user-structure).