### Overview

FastComments Image Chat extiende el widget de comentarios estándar de FastComments, por lo que hereda todas las opciones de configuración del widget base mientras añade algunas específicas para anotaciones en imágenes.

### Required Configuration

#### tenantId

Se requiere su Tenant ID de FastComments. Puede encontrar esto en su [panel de control de FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Por defecto, Image Chat genera un identificador único para cada conversación basado en la URL de la página, la fuente de la imagen y las coordenadas X/Y. Puede anular esto con un `urlId` personalizado.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Esto es útil cuando la estructura de sus URL puede cambiar pero desea mantener las mismas conversaciones, o cuando quiere compartir anotaciones a través de varias páginas.

#### chatSquarePercentage

Controla el tamaño de los marcadores de chat clicables como un porcentaje del ancho de la imagen. El valor por defecto es 5%, lo que significa que cada marcador tiene el 5% del ancho de la imagen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Marcadores más grandes y visibles
});
```

Valores más pequeños crean marcadores menos intrusivos que funcionan mejor para imágenes detalladas. Valores más grandes hacen que los marcadores sean más fáciles de ver y de hacer clic en imágenes con mucho contenido o para usuarios en dispositivos móviles.

#### hasDarkBackground

Active el estilo de modo oscuro cuando su página tiene un fondo oscuro.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Una función de callback que se ejecuta cada vez que cambia el recuento de comentarios. Esto es útil para actualizar elementos de la interfaz de usuario como insignias o títulos de página.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Dado que Image Chat extiende el widget de comentarios estándar, puede usar cualquier opción de configuración del widget base de FastComments. Aquí hay algunas opciones de uso común:

#### locale

Establezca el idioma para la interfaz del widget. FastComments soporta docenas de idiomas.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Español
});
```

#### readonly

Haga que todas las conversaciones sean de solo lectura. Los usuarios pueden ver los marcadores y las discusiones existentes pero no pueden crear nuevas ni responder.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integre con su sistema de autenticación usando Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Configuración de SSO
    }
});
```

Consulte la documentación de SSO para obtener detalles completos sobre las opciones de autenticación.

#### maxReplyDepth

Controle cuántos niveles de profundidad pueden tener las respuestas. Por defecto, Image Chat establece esto en 0, lo que significa que todos los comentarios son planos (sin respuestas anidadas). Puede cambiar esto si desea conversaciones en hilos.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Permitir 3 niveles de anidamiento
});
```

### Internal Configuration

Estas opciones se establecen automáticamente por Image Chat y no deben ser anuladas:

El `productId` se establece automáticamente en `2` para Image Chat. La extensión `floating-chat` se carga automáticamente para proporcionar la funcionalidad de la ventana de chat. El widget detecta automáticamente dispositivos móviles (pantallas de menos de 768px de ancho) y ajusta la interfaz en consecuencia con ventanas de chat a pantalla completa.

### Target Element Flexibility

El primer parámetro de `FastCommentsImageChat` puede ser un elemento `<img>` directamente o un elemento contenedor con una imagen dentro:

```javascript
// Elemento de imagen directo
FastCommentsImageChat(document.getElementById('my-image'), config);

// Contenedor con imagen en su interior
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

El widget encontrará la imagen automáticamente si pasa un elemento contenedor.

### Complete Example

Aquí hay un ejemplo que muestra múltiples opciones de configuración juntas:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Tu configuración de SSO
    },
    maxReplyDepth: 1
});
```

Para una lista completa de todas las opciones de configuración disponibles heredadas del widget base, consulte la documentación principal de configuración de FastComments.