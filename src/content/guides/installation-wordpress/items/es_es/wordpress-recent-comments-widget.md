El widget Comentarios recientes muestra los comentarios más recientes publicados en todo tu sitio. Es útil en barras laterales, pies de página o en cualquier lugar donde quieras resaltar actividad reciente para animar a seguir leyendo.

## Opciones

- **Título** (opcional): El encabezado que se muestra encima de la lista. Por defecto es "Comentarios recientes".
- **Cantidad** (opcional): Cuántos comentarios mostrar. Rango de 1 a 50. Por defecto 5.

## Cómo añadirlo

### Dentro de una entrada o página

En el editor de bloques, añade un bloque **Shortcode** y pega:

[inline-code-attrs-start title = 'Shortcode de Comentarios Recientes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

El atributo `count` acepta cualquier valor entre 1 y 50.

### En una barra lateral o pie de página (temas clásicos)

Ve a **Apariencia > Widgets** en el administrador de WordPress. En el inserto de bloques, busca "FastComments" y elige **FastComments: Comentarios recientes**. Arrástralo a una barra lateral, cabecera o área de pie de página, y luego configura el título y la cantidad desde el panel del widget.

### En un tema de bloques (Edición completa del sitio)

Abre el **Editor del sitio** bajo **Apariencia > Editor**. Navega hasta la parte de la plantilla donde debería aparecer el widget, inserta un bloque **Legacy Widget**, y selecciona **FastComments: Comentarios recientes** en el menú desplegable.

## Solución de problemas

El widget solo se renderiza después de que la configuración de FastComments esté completa y se haya almacenado un tenant ID. Si el área del widget está en blanco, completa la configuración en **FastComments** en el administrador de WordPress y recarga la página.