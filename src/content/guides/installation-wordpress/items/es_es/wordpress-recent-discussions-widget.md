El widget de Discusiones Recientes muestra las páginas de tu sitio con la actividad de comentarios más reciente. Es útil para destacar los hilos que siguen recibiendo respuestas, de modo que los visitantes puedan volver a conversaciones activas en lugar de llegar a páginas tranquilas.

## Opciones

- **Title** (opcional): El encabezado que se muestra encima de la lista. Por defecto es "Recent Discussions".
- **Count** (opcional): Cuántas discusiones mostrar. Rango de 1 a 50. Por defecto es 20.

## Cómo Añadirlo

### Dentro de una Entrada o Página

En el editor de bloques, añade un bloque **Shortcode** y pega:

[inline-code-attrs-start title = 'Shortcode de Discusiones Recientes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

El atributo `count` acepta cualquier valor entre 1 y 50.

### En una Barra Lateral o Pie de Página (Temas Clásicos)

Ve a **Appearance > Widgets** en el administrador de WordPress. Desde el insertador de bloques, busca "FastComments" y elige **FastComments: Recent Discussions**. Arrástralo a una barra lateral, encabezado o área de pie de página, y luego configura el título y el recuento desde el panel del widget.

### En un Tema de Bloques (Edición Completa del Sitio)

Abre el **Site Editor** en **Appearance > Editor**. Navega a la parte de la plantilla donde debería aparecer el widget, inserta un bloque **Legacy Widget** y selecciona **FastComments: Recent Discussions** en el menú desplegable.

## Solución de Problemas

El widget solo se muestra después de que la configuración de FastComments esté completa y se haya almacenado un ID de inquilino. Si el área del widget está en blanco, completa la configuración en **FastComments** en el administrador de WordPress y recarga la página.

Si el orden de las discusiones parece desactualizado, verifica que las páginas subyacentes hayan terminado de sincronizarse en el panel de FastComments. El widget lee datos en vivo, por lo que los comentarios recién importados pueden tardar un momento en aparecer.