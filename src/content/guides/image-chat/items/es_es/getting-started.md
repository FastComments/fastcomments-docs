### Casos de uso

Image Chat funciona muy bien para la retroalimentación de diseño cuando los equipos necesitan discutir elementos específicos en maquetas o capturas de pantalla. Los sitios de reseñas de productos pueden permitir que los clientes comenten características específicas visibles en las fotos de productos. Las plataformas educativas pueden usarlo para discutir diagramas, mapas o imágenes científicas. Las galerías de fotos pueden volverse interactivas con comentarios específicos por ubicación. Los sitios inmobiliarios pueden permitir que los visitantes discutan características específicas visibles en las fotos de las propiedades.

### Inicio rápido

Empezar con Image Chat es sencillo. Necesitas el script FastComments Image Chat, un elemento de imagen o un contenedor con una imagen, y un objeto de configuración con tu Tenant ID.

### Instalación

Agrega el script de Image Chat a tu página:

[inline-code-attrs-start title = 'Carga del script de Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implementación básica

Aquí tienes un ejemplo mínimo:

[inline-code-attrs-start title = 'Implementación básica de Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Tu imagen -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Cargar el script de Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Inicializar Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Reemplaza `'demo'` con tu Tenant ID real de FastComments si no lo está ya, que puedes encontrar en tu [panel de FastComments](https://fastcomments.com/auth/my-account).

### Cómo funciona

Una vez inicializado, los usuarios pueden hacer clic en cualquier parte de la imagen. Cuando ocurre un clic, aparece un marcador cuadrado visual en esa ubicación y se abre una ventana de chat. Otros usuarios pueden ver todos los marcadores y hacer clic en ellos para ver o participar en esas conversaciones. Todas las discusiones se sincronizan en tiempo real entre todos los visitantes.

El widget usa posicionamiento basado en porcentajes, por lo que los marcadores se mantienen en la ubicación correcta incluso cuando la imagen cambia de tamaño o se visualiza en diferentes tamaños de pantalla.

### Demostración en vivo

Puedes ver Image Chat en acción en nuestra [página de demostración en vivo](https://fastcomments.com/product/image-chat).

### Próximos pasos

Ahora que tienes lo básico funcionando, puedes personalizar la apariencia y el comportamiento en la guía de Opciones de configuración. Consulta la guía de diseño adaptable para entender cómo funciona el posicionamiento basado en porcentajes. Aprende sobre estilos y soporte de modo oscuro en la guía de personalización. Para integraciones avanzadas, explora la Referencia de la API.

### Bibliotecas frontend

Todas las bibliotecas frontend de FastComments (react, vue, angular, etc) incluyen Image Chat.