El widget Top Pages muestra las páginas con más comentarios de tu sitio. Es útil para destacar tu contenido más atractivo ante nuevos visitantes y aumentar el tiempo en el sitio.

## Opciones

- **Title** (opcional): El encabezado que se muestra encima de la lista. Por defecto es "Top Pages".

El widget Top Pages elige su propio diseño según los datos disponibles y no acepta un atributo count.

## Cómo agregarlo

### Dentro de una publicación o página

En el editor de bloques, añade un bloque **Shortcode** y pega:

[inline-code-attrs-start title = 'Shortcode de Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### En una barra lateral o pie de página (Temas clásicos)

Ve a **Apariencia > Widgets** en el administrador de WordPress. Desde el inserter de bloques, busca "FastComments" y elige **FastComments: Top Pages**. Arrástralo a una barra lateral, encabezado o área de pie de página, luego configura el título desde el panel del widget.

### En un tema de bloques (Edición completa del sitio)

Abre el **Editor del sitio** en **Apariencia > Editor**. Navega a la parte de la plantilla donde debe aparecer el widget, inserta un bloque **Legacy Widget** y selecciona **FastComments: Top Pages** en el menú desplegable.

## Solución de problemas

El widget solo se muestra después de que la configuración de FastComments esté completa y se haya almacenado un tenant ID. Si el área del widget está en blanco, completa la configuración en **FastComments** en el administrador de WordPress y recarga la página.