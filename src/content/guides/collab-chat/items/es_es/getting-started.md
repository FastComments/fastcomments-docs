### Inicio rápido

Comenzar con Collab Chat es sencillo. Necesitas el script de FastComments Collab Chat, un elemento HTML que contenga el texto que deseas anotar y un objeto de configuración con tu Tenant ID.

### Instalación

Añade el script de Collab Chat a tu página:

[inline-code-attrs-start title = 'Cargando el script de Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Implementación básica

Aquí tienes un ejemplo mínimo:

[inline-code-attrs-start title = 'Implementación básica de Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Contenedor de contenido -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Cargar el script de Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicializar Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Cómo funciona

Una vez inicializado, los usuarios pueden seleccionar cualquier texto dentro del elemento objetivo. Tras una breve demora (3,5 segundos en escritorio), aparece un aviso que les permite iniciar una discusión. Cuando se crea una discusión, aparece un resaltado visual en el texto. Otros usuarios pueden pasar el cursor o hacer clic en el resaltado para ver y participar en la discusión. Todas las discusiones se sincronizan en tiempo real entre todos los visitantes.

### Demostración en vivo

Puedes ver Collab Chat en acción en nuestra [página de demostración en vivo](https://fastcomments.com/product/collab-chat).

### Siguientes pasos

Ahora que tienes lo básico funcionando, puedes personalizar la apariencia y el comportamiento en la guía de Opciones de Configuración. Consulta la guía de Comportamiento de Selección de Texto para entender cómo funciona la selección de texto. Aprende sobre estilos y soporte de modo oscuro en la guía de Personalización. Para integraciones avanzadas, explora la Referencia de la API.

### Bibliotecas frontend

Todas las bibliotecas frontend de FastComments (react, vue, angular, etc) incluyen Collab Chat.

---