---
### Compatibilidad con modo oscuro

Image Chat incluye soporte integrado para modo oscuro. Cuando estableces `hasDarkBackground: true` en tu configuración, las ventanas de chat y los elementos de la interfaz de usuario se ajustan automáticamente para funcionar bien en fondos oscuros.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

El estilo de modo oscuro se aplica a las ventanas de chat, a los cuadrados marcadores y a todos los elementos interactivos. Si tu sitio tiene un interruptor de modo oscuro, puedes reinicializar el widget cuando cambie el modo, o usar el enfoque de clase en el body descrito más abajo.

### Modo oscuro dinámico

Si el modo oscuro de tu sitio se controla añadiendo una clase `.dark` al elemento body, la interfaz de Image Chat respetará esto automáticamente sin necesidad de reinicializar. Los estilos del widget están diseñados para responder a la presencia de esta clase.

```css
/* Tu CSS de modo oscuro */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Estilizado personalizado con CSS

Puedes personalizar la apariencia de los marcadores, las ventanas de chat y otros elementos usando CSS. El widget añade clases específicas a las que puedes dirigirte en tu hoja de estilos.

Los cuadrados y ventanas de chat usan el sistema de estilo de burbujas de comentarios de FastComments, por lo que cualquier personalización que hayas aplicado al widget de comentarios estándar también afectará a Image Chat.

### Tamaño de los cuadrados de chat

La opción `chatSquarePercentage` controla el tamaño de los marcadores clicables. El valor predeterminado es el 5% del ancho de la imagen:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Cuadrados más grandes y más visibles
});
```

Valores más pequeños crean marcadores más sutiles que se integran en la imagen. Valores más grandes hacen que los marcadores sean más prominentes y más fáciles de pulsar, especialmente en dispositivos móviles o por motivos de accesibilidad.

### Comportamiento en móviles

En pantallas de menos de 768px de ancho, Image Chat cambia automáticamente a un diseño optimizado para móviles. Las ventanas de chat aparecen a pantalla completa en lugar de flotar junto a los marcadores, ofreciendo una mejor usabilidad en pantallas pequeñas.

Los marcadores permanecen visibles en sus posiciones responsivas sobre la imagen. Los usuarios pueden tocar cualquier marcador para abrir la interfaz de chat a pantalla completa. Este comportamiento está integrado y no requiere ninguna configuración.

### Apariencia de la ventana de chat

Las ventanas de chat tienen 300px de ancho en escritorio con una flecha de 16px apuntando al marcador. Las ventanas se posicionan automáticamente según el espacio disponible en la ventana de visualización, usando clases de posicionamiento como `to-right`, `to-left`, `to-top` y `to-bottom`.

Puedes añadir CSS personalizado para ajustar colores, fuentes, espaciado u otras propiedades visuales de estas ventanas. Las ventanas de chat usan la misma estructura de componentes que el widget estándar de FastComments, por lo que heredan cualquier personalización global que hayas aplicado.

### Inicialización perezosa

Las ventanas de chat se inicializan al pasar el ratón para usuarios de escritorio o inmediatamente cuando se crean. Esto reduce la sobrecarga de carga inicial al solo renderizar la interfaz de chat cuando los usuarios realmente interactúan con un marcador.

La inicialización perezosa ocurre de forma transparente. Los usuarios no notan ningún retraso, pero el navegador no necesita renderizar docenas de ventanas de chat ocultas si tienes muchos marcadores en una imagen.

### Localización

Image Chat admite todas las mismas opciones de localización que el widget estándar de FastComments. Establece la opción `locale` para mostrar el texto de la interfaz en diferentes idiomas:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francés
});
```

FastComments admite decenas de idiomas. La configuración de locale afecta todo el texto de la interfaz, incluyendo los avisos, botones y texto de marcador de posición.

### Opciones de personalización heredadas

Dado que Image Chat extiende el widget de comentarios estándar, hereda todas las opciones de personalización del widget base. Esto incluye clases CSS personalizadas, traducciones personalizadas, personalización de avatares, formato de fechas y mucho más.

Consulta la documentación principal de personalización de FastComments para la lista completa de opciones de personalización disponibles.

### Uso de fuentes personalizadas

Si tu sitio usa fuentes personalizadas, la interfaz de Image Chat heredará esas fuentes del CSS de tu página. Las ventanas de chat se renderizan dentro del DOM de tu página y respetan la configuración tipográfica existente.

Para mejores resultados, asegúrate de que tus fuentes personalizadas estén cargadas antes de inicializar Image Chat, o acepta que puede haber un breve parpadeo de texto sin estilo mientras las fuentes se cargan.

### Diseño visual de los marcadores

Los marcadores cuadrados tienen un diseño visual sutil que los hace visibles sin abrumar la imagen. Puedes personalizar su apariencia con CSS si deseas un tratamiento visual diferente.

Los marcadores incluyen estados de hover que proporcionan retroalimentación cuando los usuarios mueven el cursor sobre ellos. En dispositivos táctiles, la interacción de toque proporciona retroalimentación inmediata abriendo la ventana de chat.

---