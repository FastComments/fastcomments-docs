[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utiliza cifrado HMAC-SHA256 como el mecanismo para implementar SSO. Primero repasaremos la arquitectura general, proporcionaremos ejemplos y pasos detallados.

También hay documentación relativa a la migración desde otros proveedores con mecanismos SSO similares, y las diferencias.

El flujo se ve así:

<div class="screenshot white-bg">
    <div class="title">Flujo SSO seguro</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagrama SSO seguro" />
</div>

Dado que Secure SSO implica desarrollo full-stack, ejemplos completos y funcionales en Java/Spring, NodeJS/Express y PHP puro están actualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">en GitHub</a>.

Aunque usamos ExpressJS en el ejemplo de NodeJS y Spring en el ejemplo de Java, no se requieren frameworks/bibliotecas en estos entornos para implementar FastComments SSO: los paquetes de criptografía nativos funcionan.

No tienes que escribir nuevos endpoints de API con FastComments SSO. Simplemente encripta la información del usuario usando tu clave secreta y pasa la carga útil (payload) al widget de comentarios.

#### Obtener tu API Secret Key

Tu API Secret puede recuperarse desde <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">esta página</a>. También puedes encontrar esta página yendo a My Account, haciendo clic en el mosaico API/SSO y luego en "Get API Secret Key".

#### Parámetros del widget de comentarios

La documentación de alto nivel de la API para el widget de comentarios se puede encontrar <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">aquí</a>.

Entremos en más detalle sobre lo que significan estos parámetros.

El widget de comentarios toma un objeto de configuración: ya pasas esto si estás usando FastComments para pasar tu customer id (llamado tenantId).

Para habilitar SSO, pasa un nuevo objeto "sso", que debe tener los siguientes parámetros. Los valores deben generarse en el lado del servidor.

- userDataJSONBase64: Los datos del usuario en formato JSON, que luego se codifican en Base64.
- verificationHash: El hash HMAC-SHA256 creado a partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Marca de tiempo Epoch, en **milisegundos**. No debe estar en el futuro, ni tener más de dos días de antigüedad.
- loginURL: Una URL que el widget de comentarios puede mostrar para iniciar sesión el usuario.
- logoutURL: Una URL que el widget de comentarios puede mostrar para cerrar sesión el usuario.
- loginCallback: Cuando se proporciona en lugar de la login URL, una función que el widget de comentarios invocará al hacer clic en el botón de inicio de sesión.
- logoutCallback: Cuando se proporciona en lugar de la logout URL, una función que el widget de comentarios invocará al hacer clic en el botón de cierre de sesión.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'El objeto de usuario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
}
[inline-code-end]

#### Moderators and Administrators

Para administradores y moderadores, pasa las banderas respectivas `isAdmin` o `isModerator` en el objeto `SSOUser`.

#### Notifications

Para habilitar o deshabilitar las notificaciones, establece el valor de `optedInNotifications` en `true` o `false` respectivamente. La primera vez que el usuario cargue la página con este valor en la carga SSO, sus ajustes de notificación se actualizarán.

Además, si quieres que los usuarios reciban correos electrónicos de notificación por actividad en páginas a las que están suscritos (en lugar de solo notificaciones dentro de la aplicación), establece `optedInSubscriptionNotifications` en `true`.

#### VIP Users & Special Labels

Puedes mostrar una etiqueta especial junto al nombre del usuario usando el campo opcional "displayLabel".

#### Unauthenticated users

Para representar a un usuario no autenticado, simplemente no rellenes userDataJSONBase64, verificationHash ni timestamp. Proporciona una loginURL.

Estos usuarios no podrán comentar y en su lugar se les mostrará un mensaje de inicio de sesión (mensaje, enlace o botón, dependiendo de la configuración).

#### Direct Examples for Serializing and Hashing User Data

Más detalles y ejemplos directos <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">aquí</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">aquí</a> (java) y <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">aquí</a> (php).

Entendemos que cualquier integración puede ser un proceso complicado y doloroso. No dudes en ponerte en contacto con tu representante o usar la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">página de soporte</a>.