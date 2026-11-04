### Descripción general

FastComments Image Chat extiende el widget de comentarios estándar de FastComments, por lo que hereda todas las opciones de configuración del widget base, además de añadir algunas específicas para anotaciones de imágenes.

### Configuración requerida

#### tenantId

Se requiere su Tenant ID de FastComments. Puede encontrarlo en su [panel de FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opciones específicas de Image Chat

#### urlId

Por defecto, Image Chat genera un identificador único para cada conversación basado en la URL de la página, la fuente de la imagen y las coordenadas X/Y. Puede sobrescribir esto con un `urlId` personalizado.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Esto es útil cuando la estructura de su URL puede cambiar pero desea mantener las mismas conversaciones, o cuando desea compartir anotaciones entre varias páginas.

#### chatSquarePercentage

Controla el tamaño de los marcadores de chat clicables como porcentaje del ancho de la imagen. El valor predeterminado es 5%, lo que significa que cada marcador es el 5% del ancho de la imagen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Marcadores más grandes y visibles
});
```

Los valores más pequeños crean marcadores menos intrusivos que funcionan mejor en imágenes detalladas. Los valores más grandes hacen que los marcadores sean más fáciles de ver y hacer clic en imágenes con mucho contenido o para usuarios de dispositivos móviles.

#### hasDarkBackground

Habilite el estilo de modo oscuro cuando su página tenga un fondo oscuro.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Una función de devolución de llamada que se activa cada vez que cambia el número de comentarios. Esto es útil para actualizar elementos de la interfaz como insignias o títulos de página.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Opciones de configuración heredadas

Dado que Image Chat extiende el widget de comentarios estándar, puede usar cualquier opción de configuración del widget base de FastComments. Aquí hay algunas opciones de uso común:

#### locale

Establece el idioma de la interfaz del widget. FastComments admite docenas de idiomas.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Español
});
```

#### readonly

Convierte todas las conversaciones en de solo lectura. Los usuarios pueden ver los marcadores y discusiones existentes, pero no pueden crear nuevos ni responder.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso y simpleSSO

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

Controla cuántos niveles de profundidad pueden tener las respuestas. Por defecto, Image Chat establece esto en 0, lo que significa que todos los comentarios son planos (sin respuestas anidadas). Puede cambiar esto si desea conversaciones en hilo.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Permitir 3 niveles de anidación
});
```

### Configuración interna

Estas opciones son establecidas automáticamente por Image Chat y no deben sobrescribirse:

El `productId` se establece automáticamente en `2` para Image Chat. La extensión `floating-chat` se carga automáticamente para proporcionar la funcionalidad de la ventana de chat. El widget detecta automáticamente los dispositivos móviles (pantallas de menos de 768px de ancho) y ajusta la interfaz en consecuencia con ventanas de chat a pantalla completa.

### Flexibilidad del elemento de destino

El primer parámetro de `FastCommentsImageChat` puede ser un elemento `<img>` directamente o un elemento contenedor con una imagen dentro:

```javascript
// Elemento de imagen directo
FastCommentsImageChat(document.getElementById('my-image'), config);

// Contenedor con imagen dentro
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

El widget encontrará la imagen automáticamente si pasa un elemento contenedor.

### Ejemplo completo

Aquí hay un ejemplo que muestra varias opciones de configuración juntas:

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
        // Su configuración de SSO
    },
    maxReplyDepth: 1
});
```

Para obtener una lista completa de todas las opciones de configuración disponibles heredadas del widget base, consulte la documentación principal de configuración de FastComments.