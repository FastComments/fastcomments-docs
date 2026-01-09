### Soporte de Modo Oscuro

### Modo Oscuro Dinámico

Si el modo oscuro de tu sitio se controla agregando una clase `.dark` al elemento body, la interfaz de Collab Chat respetará esto automáticamente sin requerir reinitialización. Los estilos del widget están diseñados para responder a la presencia de esta clase.

[inline-code-attrs-start title = 'Ejemplo de CSS de modo oscuro'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Tu CSS de modo oscuro */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Estilizado personalizado con CSS

Puedes personalizar la apariencia de los resaltados, las ventanas de chat y otros elementos usando CSS. El widget añade clases específicas a las que puedes dirigirte en tu hoja de estilos.

Los resaltados de texto usan el sistema de estilo de burbujas de comentario de FastComments, por lo que cualquier personalización que hayas aplicado al widget de comentarios estándar también afectará a Collab Chat.

### Personalización de la barra superior

La barra superior muestra el número de usuarios en línea y el número de discusiones. Puedes personalizar su posición proporcionando un elemento personalizado como `topBarTarget`:

[inline-code-attrs-start title = 'Ubicación personalizada de la barra superior'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

O desactívala por completo estableciéndola en `null`:

[inline-code-attrs-start title = 'Desactivar la barra superior'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Comportamiento móvil

En pantallas de menos de 768px de ancho, Collab Chat cambia automáticamente a una disposición optimizada para móvil. Las ventanas de chat aparecen a pantalla completa en lugar de flotar junto al texto, y se elimina el retraso de selección para una interacción más inmediata.

Este comportamiento está integrado y no requiere ninguna configuración. El widget detecta el tamaño de la pantalla automáticamente y se ajusta en consecuencia.

### Apariencia de la ventana de chat

Las ventanas de chat tienen 410px de ancho en escritorio con una flecha de 16px apuntando al texto resaltado. Las ventanas se posicionan automáticamente según el espacio disponible en la ventana gráfica, usando clases de posicionamiento como `to-right`, `to-left`, `to-top`, y `to-bottom`.

Puedes añadir CSS personalizado para ajustar colores, fuentes, espaciado u otras propiedades visuales de estas ventanas. Las ventanas de chat usan la misma estructura de componentes que el widget estándar de FastComments, por lo que heredan cualquier personalización global que hayas aplicado.

### Localización

Collab Chat admite todas las mismas opciones de localización que el widget estándar de FastComments. Establece la opción `locale` para mostrar el texto de la interfaz en diferentes idiomas:

[inline-code-attrs-start title = 'Establecer locale'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Español
});
[inline-code-end]

FastComments admite docenas de idiomas. La configuración de locale afecta todo el texto de la interfaz, incluidos los mensajes, botones y texto de marcador de posición.

### Opciones de personalización heredadas

Como Collab Chat extiende el widget de comentarios estándar, hereda todas las opciones de personalización del widget base. Esto incluye clases CSS personalizadas, traducciones personalizadas, personalización de avatar, formato de fechas y mucho más.

Consulta la documentación principal de personalización de FastComments para la lista completa de opciones de personalización disponibles.

### Trabajar con fuentes personalizadas

Si tu sitio usa fuentes personalizadas, la interfaz de Collab Chat heredará esas fuentes desde el CSS de tu página. Es posible que tengas que crear una regla de personalización de widget e `@import` cualquier fuente en el CSS personalizado dentro de esa regla si deseas que la ventana de chat flotante use las mismas fuentes.