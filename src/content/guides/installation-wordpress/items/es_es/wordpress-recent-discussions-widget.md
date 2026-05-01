El widget Recent Discussions muestra las páginas de tu sitio con la actividad de comentarios más reciente. Es útil para resaltar hilos que todavía están recibiendo aportes, de modo que los visitantes puedan volver a conversaciones activas en lugar de llegar a páginas tranquilas.

## Options

- **Title** (opcional): El encabezado que se muestra encima de la lista. Por defecto: "Discusiones recientes".
- **Count** (opcional): Cuántas discusiones mostrar. Rango de 1 a 50. Por defecto: 20.

## How to Add It

### Inside a Post or Page

En el editor de bloques, añade un bloque **Shortcode** y pega:

[inline-code-attrs-start title = 'Shortcode de Discusiones Recientes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

El atributo `count` acepta cualquier valor entre 1 y 50.

### In a Sidebar or Footer (Classic Themes)

Ve a **Apariencia > Widgets** en el administrador de WordPress. Desde el inserter de bloques, busca "FastComments" y elige **FastComments: Recent Discussions**. Arrástralo a una barra lateral, encabezado o área de pie de página, y luego configura el título y la cantidad desde el panel del widget.

### In a Block Theme (Full Site Editing)

Abre el **Editor del sitio** en **Apariencia > Editor**. Navega a la parte de la plantilla donde debe aparecer el widget, inserta un bloque **Legacy Widget** y selecciona **FastComments: Recent Discussions** del desplegable.

## Troubleshooting

El widget solo se renderiza después de que la configuración de FastComments esté completa y se almacene un tenant ID. Si el área del widget está en blanco, completa la configuración en **FastComments** en el administrador de WordPress y recarga la página.

Si el orden de las discusiones parece desactualizado, verifica que las páginas subyacentes hayan terminado de sincronizarse en el panel de FastComments. El widget lee datos en vivo, por lo que los comentarios recién importados pueden tardar un momento en aparecer.