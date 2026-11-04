[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utiliza el cifrado HMAC-SHA256 como mecanismo para implementar SSO. Primero repasaremos la arquitectura general, proporcionaremos ejemplos y los pasos detallados.

También existe documentación sobre la migración desde otros proveedores con mecanismos de SSO similares, y las diferencias.

El flujo se ve así:

<div class="screenshot white-bg">
    <div class="title">Flujo de Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagrama de Secure SSO" />
</div>

Dado que Secure SSO implica desarrollo full-stack, hay ejemplos de código completos y funcionales en Java/Spring, NodeJS/Express y PHP vanilla disponibles actualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">en GitHub</a>.

Aunque usamos ExpressJS en el ejemplo de NodeJS y Spring en el ejemplo de Java, no se requieren frameworks/librerías en estos entornos de ejecución para implementar FastComments SSO - los paquetes de criptografía nativos funcionan.

No tienes que escribir ningún endpoint de API nuevo con FastComments SSO. Simplemente cifra la información del usuario usando tu clave secreta y pasa el payload al widget de comentarios.

#### Obtén Tu Clave Secreta de API

Tu API Secret se puede obtener en <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">esta página</a>. También puedes encontrar esta página yendo a Mi Cuenta, haciendo clic en el mosaico API/SSO, y luego haciendo clic en "Get API Secret Key".

#### Parámetros del Widget de Comentarios

La documentación de la API de alto nivel para el widget de comentarios se puede encontrar <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">aquí</a>.

Entremos en más detalle sobre lo que significan estos parámetros.

El widget de comentarios recibe un objeto de configuración - ya lo estás pasando si usas FastComments para pasar tu id de cliente (llamado tenantId).

Para habilitar SSO, pasa un nuevo objeto "sso", que debe tener los siguientes parámetros. Los valores deben generarse en el lado del servidor.

- userDataJSONBase64: Los datos del usuario en formato JSON, que luego se codifican en Base64.
- verificationHash: El hash HMAC-SHA256 creado a partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Marca de tiempo de época, en **milisegundos**. No debe estar en el futuro, ni con más de dos días de antigüedad.
- loginURL: Una URL que el widget de comentarios puede mostrar para iniciar sesión del usuario.
- logoutURL: Una URL que el widget de comentarios puede mostrar para cerrar sesión del usuario.
- loginCallback: Cuando se proporciona en lugar de la URL de inicio de sesión, una función que el widget de comentarios invocará al hacer clic en el botón de inicio de sesión.
- logoutCallback: Cuando se proporciona en lugar de la URL de cierre de sesión, una función que el widget de comentarios invocará al hacer clic en el botón de cierre de sesión.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Código Cliente de Secure SSO'; isFunctional = false; code-example-end]

#### El Objeto de Usuario

El objeto User contiene el siguiente esquema:
[inline-code-attrs-start title = 'El Objeto de Usuario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of global badge IDs to assign. Limited to 30 badges. Order is respected. **/
        badgeIds: string[];
        /** Array of badge IDs scoped to the current page (urlId). Only displayed on the assigned page. **/
        pageBadgeIds?: string[];
        /** If true, replaces existing displayed badges. Global and page-scoped are overridden independently. **/
        override?: boolean;
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderadores y Administradores

Para administradores y moderadores, pasa los flags respectivos `isAdmin` o `isModerator` en el objeto `SSOUser`.

#### Notificaciones

Para habilitar o deshabilitar las notificaciones, establece el valor de `optedInNotifications` en `true` o `false` respectivamente. La primera vez que el usuario cargue la página con este valor en el payload de SSO, su configuración de notificaciones se actualizará.

Además, si quieres que los usuarios reciban correos de notificación por actividad en páginas a las que se suscribieron (a diferencia de solo notificaciones dentro de la aplicación), entonces establece `optedInSubscriptionNotifications` en `true`.

#### Usuarios VIP y Etiquetas Especiales

Puedes mostrar una etiqueta especial junto al nombre del usuario usando el campo opcional "displayLabel".

#### Usuarios no autenticados

Para representar un usuario no autenticado, simplemente no rellenes userDataJSONBase64, verificationHash, ni timestamp. Proporciona un loginURL.

Estos usuarios no podrán comentar, y en su lugar se les presentará un mensaje de inicio de sesión (mensaje, enlace o botón, dependiendo de la configuración).

#### Ejemplos Directos para Serializar y Hashear Datos de Usuario

Más detalles como ejemplo <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">aquí</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">aquí</a> (java) y <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">aquí</a> (php).

Entendemos que cualquier integración puede ser un proceso complicado y doloroso. No dudes en contactar a tu representante o usar la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">página de soporte</a>.