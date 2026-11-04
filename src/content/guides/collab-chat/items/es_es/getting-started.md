### Inicio rápido

Empezar con Collab Chat es sencillo. Necesitas el script de FastComments Collab Chat, un elemento HTML que contenga el texto que deseas anotar y un objeto de configuración con tu Tenant ID.

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
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Reemplaza `'demo'` con tu Tenant ID real de FastComments si aún no lo has hecho, el cual puedes encontrar en tu [panel de FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Cómo funciona

Una vez inicializado, los usuarios pueden seleccionar cualquier texto dentro del elemento objetivo. Tras una breve pausa (3.5 segundos en escritorio), aparece un mensaje que les permite iniciar una discusión. Cuando se crea una discusión, aparece un resaltado visual sobre el texto. Otros usuarios pueden pasar el cursor sobre el resaltado o hacer clic en él para ver y participar en la discusión. Todas las discusiones se sincronizan en tiempo real entre todos los visitantes.

### Demo en vivo

Puedes ver Collab Chat en acción en nuestra [página de demostración en vivo](https://fastcomments.com/product/collab-chat).

### Próximos pasos

Ahora que tienes lo básico funcionando, puedes personalizar la apariencia y el comportamiento en la guía de Opciones de configuración. Consulta la guía de Comportamiento de selección de texto para entender cómo funciona la selección de texto. Aprende sobre los estilos y el soporte del modo oscuro en la guía de Personalización. Para integraciones avanzadas, explora la Referencia de la API.

### Librerías frontend

Todas las librerías frontend de FastComments (react, vue, angular, etc.) incluyen Collab Chat.