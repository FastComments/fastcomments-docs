### Cómo funciona la selección de texto

Cuando los usuarios seleccionan texto dentro del contenedor Collab Chat, el widget captura esa selección y les permite iniciar una discusión. La selección puede ser tan pequeña como una sola palabra o tan grande como varios párrafos que abarcan diferentes elementos.

### Retardo de selección

En dispositivos de escritorio, hay un retraso de 3,5 segundos entre el momento en que un usuario selecciona texto y el momento en que aparece el aviso de discusión. Esto evita que la interfaz parpadee cuando los usuarios simplemente están resaltando texto para copiar o leer. En dispositivos móviles, el aviso aparece de inmediato ya que la selección de texto es más deliberada en pantallas táctiles.

### IDs de conversación únicos

Cada conversación recibe un `urlId` único que combina la URL de la página, la ruta del elemento del DOM y el rango de texto serializado. Esto asegura que cada selección de texto cree una conversación distinta que se pueda encontrar más tarde.

Si proporciona un `urlId` personalizado en su configuración, se combinará con la ruta del elemento y el rango de texto para crear el identificador final.

### Resaltados visuales

Cuando existe una discusión para una selección de texto determinada, ese texto recibe un resaltado visual. El resaltado se implementa mediante colores de fondo y aparece al pasar el cursor o cuando la ventana de chat asociada está abierta.

El sistema de resaltado funciona envolviendo el texto seleccionado en un elemento especial que puede ser estilizado. Este enfoque garantiza que los resaltados sigan siendo precisos incluso cuando la estructura HTML subyacente es compleja.

### Posicionamiento de la ventana de chat

Cuando un usuario hace clic en un resaltado o crea una nueva anotación, aparece una ventana de chat cerca del texto seleccionado. El widget calcula automáticamente la mejor posición para esta ventana en función del espacio disponible en la ventana gráfica.

El sistema de posicionamiento utiliza clases CSS como `to-right`, `to-left`, `to-top` y `to-bottom` para indicar en qué dirección debe extenderse la ventana de chat desde el resaltado. En dispositivos móviles (pantallas de menos de 768px de ancho), la ventana de chat siempre aparece en pantalla completa para una mejor usabilidad.

### Dimensiones de la ventana de chat

Las ventanas de chat tienen 410px de ancho en escritorio con 20px de separación y una flecha visual de 16px que apunta al texto resaltado. Estas dimensiones son fijas para asegurar un comportamiento coherente, pero puede personalizar la apariencia con CSS.

### Selecciones entre elementos

Los usuarios pueden seleccionar texto que abarca múltiples elementos HTML, como resaltar desde la mitad de un párrafo hasta el inicio de otro. El sistema de serialización de rangos maneja esto correctamente y resaltará todo el texto seleccionado incluso a través de los límites de los elementos.

### Compatibilidad con navegadores

El sistema de selección de texto utiliza la API estándar `window.getSelection()` que es compatible con todos los navegadores modernos. Para versiones antiguas de Internet Explorer, recurre a `document.selection` para compatibilidad.

### Persistencia de la selección

Una vez que se crea una conversación para una selección de texto, esa anotación persiste incluso si se recarga la página. El rango serializado y la ruta del DOM permiten que el widget restaure los resaltados en la misma ubicación exacta cuando los usuarios regresen a la página.

Esto funciona de manera fiable siempre que el contenido de su página permanezca estable. Si cambia el contenido de texto o reestructura su HTML, las anotaciones existentes pueden dejar de alinearse correctamente con el texto. Por esta razón, es mejor evitar cambios importantes en el contenido de las páginas con anotaciones activas, o considerar migrar las anotaciones cuando sean necesarios cambios en el contenido.