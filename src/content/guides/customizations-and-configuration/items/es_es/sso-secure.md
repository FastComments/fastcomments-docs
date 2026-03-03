[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utiliza encriptación HMAC-SHA256 como el mecanismo para implementar SSO. Primero revisaremos la arquitectura general, proporcionaremos ejemplos y pasos detallados.

También hay documentación sobre la migración desde otros proveedores con mecanismos de SSO similares, y las diferencias.

El flujo se ve así:

<div class="screenshot white-bg">
    <div class="title">Flujo SSO Seguro</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagrama SSO Seguro" />
</div>

Dado que Secure SSO implica desarrollo full-stack, ejemplos de código completos y funcionales en Java/Spring, NodeJS/Express y PHP vanilla están actualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">en GitHub</a>.

Aunque usamos ExpressJS en el ejemplo de NodeJS y Spring en el ejemplo de Java, no se requieren frameworks/bibliotecas en esos entornos de ejecución para implementar FastComments SSO: los paquetes criptográficos nativos funcionan.

No tienes que crear nuevos endpoints de API con FastComments SSO. Simplemente cifra la información del usuario usando tu clave secreta y pasa la carga útil al widget de comentarios.

#### Get Your API Secret Key

Tu API Secret puede recuperarse desde <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">esta página</a>. También puedes encontrar esta página yendo a My Account, haciendo clic en el mosaico API/SSO y luego en "Get API Secret Key".

#### Comment Widget Parameters

La documentación de alto nivel de la API para el widget de comentarios se puede encontrar <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">aquí</a>.

Vamos a detallar qué significan estos parámetros.

El widget de comentarios toma un objeto de configuración: ya lo pasas si estás usando FastComments para enviar tu id de cliente (llamado tenantId).

Para habilitar SSO, pasa un nuevo objeto "sso", que debe tener los siguientes parámetros. Los valores deben generarse en el servidor.

- userDataJSONBase64: Los datos del usuario en formato JSON, que luego se codifican en Base64.
- verificationHash: El hash HMAC-SHA256 creado a partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Marca de tiempo epoch, en **milisegundos**. No debe estar en el futuro, ni más de dos días en el pasado.
- loginURL: Una URL que el widget de comentarios puede mostrar para que el usuario inicie sesión.
- logoutURL: Una URL que el widget de comentarios puede mostrar para que el usuario cierre sesión.
- loginCallback: Cuando se proporciona en lugar de la URL de inicio de sesión, una función que el widget de comentarios invocará al hacer clic en el botón de inicio de sesión.
- logoutCallback: Cuando se proporciona en lugar de la URL de cierre de sesión, una función que el widget de comentarios invocará al hacer clic en el botón de cierre de sesión.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'El objeto de usuario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Requerido. Máx. 1k caracteres. **/
    id: string;
    /** Requerido. Máx. 1k caracteres. Nota: Debe ser único. **/
    email: string;
    /** Requerido. Máx. 1k caracteres. Nota: El nombre de usuario no puede ser un correo electrónico. No tiene que ser único. **/
    username: string;
    /** Opcional. Máx. 3k caracteres para URLs. Por defecto se obtiene de gravatar basado en el correo electrónico. Soporta imágenes codificadas en base64, en cuyo caso el límite es 50k caracteres. **/ 
    avatar?: string;
    /** Opcional. Por defecto false. **/
    optedInNotifications?: boolean;
    /** Opcional. Por defecto false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcional. Máx. 100 caracteres. Esta etiqueta se mostrará junto a su nombre. Por defecto es Administrador/Moderador cuando corresponda. **/
    displayLabel?: string;
    /** Opcional. Máx. 500 caracteres. Esto se mostrará en lugar del nombre de usuario. **/
    displayName?: string;
    /** Opcional. Máx. 2k caracteres. El nombre del usuario enlazará a esto. **/
    websiteUrl?: string;
    /** Opcional. Hasta 100 grupos por usuario. Un id de grupo no puede exceder los 50 caracteres. **/
    groupIds?: string[];
    /** Opcional. Indica que el usuario es administrador. **/
    isAdmin?: boolean;
    /** Opcional. Indica que el usuario es moderador. **/
    isModerator?: boolean;
    /** Opcional, por defecto true. Establecer a false para habilitar la pestaña "activity" en el perfil del usuario. **/
    isProfileActivityPrivate?: boolean;
    /** Opcional, por defecto false. Establecer a true para desactivar los comentarios de perfil. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcional, por defecto false. Establecer a true para desactivar el envío de mensajes directos a este usuario. **/
    isProfileDMDisabled?: boolean;
    /** Configuración opcional para insignias de usuario. **/
    badgeConfig?: {
        /** Array de IDs de insignias globales para asignar. Limitado a 30 insignias. Se respeta el orden. **/
        badgeIds: string[];
        /** Array de IDs de insignias con alcance en la página actual (urlId). Solo se muestran en la página asignada. **/
        pageBadgeIds?: string[];
        /** Si es true, reemplaza las insignias mostradas existentes. Las globales y las específicas de página se sobrescriben de forma independiente. **/
        override?: boolean;
        /** Si es true, actualiza las propiedades de visualización de la insignia desde la configuración del tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Para administradores y moderadores, pasa las banderas respectivas `isAdmin` o `isModerator` en el objeto `SSOUser`.

#### Notifications

Para habilitar o deshabilitar las notificaciones, establece el valor de `optedInNotifications` a `true` o `false` respectivamente. La primera vez que el usuario carga la página con este valor en la carga útil SSO, su configuración de notificaciones se actualizará.

Adicionalmente, si quieres que los usuarios reciban correos electrónicos de notificación por actividad en páginas a las que se suscribieron (en lugar de solo notificaciones dentro de la aplicación), establece `optedInSubscriptionNotifications` en `true`.

#### VIP Users & Special Labels

Puedes mostrar una etiqueta especial junto al nombre del usuario usando el campo opcional "displayLabel".

#### Unauthenticated users

Para representar un usuario no autenticado, simplemente no completes userDataJSONBase64, verificationHash ni timestamp. Proporciona un loginURL.

Estos usuarios no podrán comentar y, en su lugar, se les presentará un mensaje de inicio de sesión (mensaje, enlace o botón, dependiendo de la configuración).

#### Direct Examples for Serializing and Hashing User Data

Más detalles y ejemplos <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">aquí</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">aquí</a> (java) y <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">aquí</a> (php).

Entendemos que cualquier integración puede ser un proceso complicado y doloroso. No dudes en contactar a tu representante o usar la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">página de soporte</a>.