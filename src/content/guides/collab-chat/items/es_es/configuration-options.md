### Descripción general

FastComments Collab Chat amplía el widget de comentarios estándar de FastComments, por lo que hereda todas las opciones de configuración del widget base y añade algunas específicas para anotaciones de texto.

### Configuración requerida

#### tenantId

Se requiere su ID de inquilino (Tenant ID) de FastComments. Puede encontrarlo en su [panel de FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opciones específicas de Collab Chat

#### urlId

Por defecto, Collab Chat genera un identificador único para cada conversación basado en la URL de la página, la ruta DOM al elemento y el rango de texto seleccionado. Puede sobrescribir esto con un `urlId` personalizado.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Esto es útil cuando la estructura de sus URL puede cambiar pero quiere mantener las mismas conversaciones, o cuando desea compartir anotaciones entre varias páginas.

#### topBarTarget

Controla la visualización de la barra superior que muestra el recuento de usuarios y el recuento de discusiones. Establézcalo en `null` para desactivar la barra superior por completo, o proporcione un elemento DOM para renderizarla en una ubicación específica.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Desactivar la barra superior
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Mostrar la barra superior en una ubicación personalizada
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Habilite el estilo de modo oscuro cuando su página tenga un fondo oscuro. Esta detección es automática, pero puede ser deseable sobrescribirla.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Una función de callback que se dispara cada vez que cambia el recuento de comentarios. Esto es útil para actualizar elementos de la UI como insignias o títulos de página.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Opciones de configuración heredadas

Dado que Collab Chat extiende el widget de comentarios estándar, puede usar cualquier opción de configuración del widget base de FastComments. Aquí hay algunas opciones comúnmente utilizadas:

#### locale

Establece el idioma para la interfaz del widget. FastComments admite docenas de idiomas.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Español
});
[inline-code-end]

#### readonly

Haga que todas las conversaciones sean de solo lectura. Los usuarios pueden ver las anotaciones existentes pero no pueden crear nuevas ni responder.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integre con su sistema de autenticación usando Single Sign-On.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Configuración de SSO
    }
});
[inline-code-end]

Consulte la documentación de SSO para obtener detalles completos sobre las opciones de autenticación.

#### maxReplyDepth

Controle cuántos niveles de profundidad pueden tener las respuestas. Por defecto, Collab Chat establece esto en 0, lo que significa que todos los comentarios son planos (sin respuestas anidadas). Puede cambiar esto si desea conversaciones en hilos.

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Permitir 3 niveles de anidamiento
});
[inline-code-end]

### Configuración interna

Estas opciones son establecidas automáticamente por Collab Chat y no deben ser sobrescritas:

El `productId` se establece automáticamente en `3` para Collab Chat. La extensión `floating-chat` se carga automáticamente para proporcionar la funcionalidad de la ventana de chat. El widget detecta automáticamente dispositivos móviles (pantallas de menos de 768px de ancho) y ajusta la interfaz en consecuencia.

### Ejemplo completo

Aquí hay un ejemplo que muestra múltiples opciones de configuración juntas:

[inline-code-attrs-start title = "Ejemplo de configuración"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Tu configuración de SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Para una lista completa de todas las opciones de configuración disponibles heredadas del widget base, consulte la documentación principal de configuración de FastComments.