### Ejemplo básico

La forma más simple de usar Image Chat es apuntar a un único elemento de imagen. Este ejemplo muestra cómo habilitar discusiones interactivas en una imagen:

[inline-code-attrs-start title = 'Ejemplo básico de Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Ejemplo con elemento contenedor

También puedes pasar un elemento contenedor que tenga una imagen dentro:

[inline-code-attrs-start title = 'Image Chat con contenedor'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Ejemplo con ID de URL personalizado

Por defecto, Image Chat usa la URL de la página combinada con la fuente de la imagen y las coordenadas para identificar conversaciones. Puedes proporcionar un `urlId` personalizado:

[inline-code-attrs-start title = 'Image Chat con ID de URL personalizado'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Esto es útil si la estructura de tus URL cambia pero quieres mantener las mismas conversaciones, o si quieres compartir los mismos puntos de discusión en varias páginas.

### Ejemplo con modo oscuro

Si tu sitio tiene un fondo oscuro y el widget no lo detecta automáticamente como debería, podemos habilitar manualmente el soporte de modo oscuro:

[inline-code-attrs-start title = 'Image Chat con modo oscuro'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Ejemplo con tamaño personalizado del cuadro de chat

Puedes ajustar el tamaño de los cuadrados clicables que aparecen en la imagen. El tamaño se especifica como un porcentaje del ancho de la imagen:

[inline-code-attrs-start title = 'Image Chat con tamaño de cuadrado personalizado'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Cuadrados más pequeños (el valor por defecto es 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Ejemplo con callback de recuento de comentarios

Haz seguimiento cuando se agregan o actualizan comentarios usando el callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat con callback de recuento de comentarios'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Ejemplo con múltiples imágenes

Puedes inicializar Image Chat en múltiples imágenes. Cada imagen tendrá sus propios puntos de discusión independientes:

[inline-code-attrs-start title = 'Image Chat en múltiples imágenes'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inicializar en la primera imagen
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inicializar en la segunda imagen
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---