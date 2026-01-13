[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Por defecto, el widget de comentarios de FastComments detectará automáticamente el modo oscuro en la mayoría de los sitios.

Cuando se detecta el modo oscuro, FastComments cambiará de texto negro sobre fondos blancos a texto blanco sobre fondo negro. Las imágenes también cambiarán.

Al cargar la página, el widget intentará determinar cuán oscuro es el fondo de la página detrás del widget de comentarios. Esto significa que
la página podría tener un fondo blanco, pero si coloca el widget de comentarios dentro de un contenedor con un fondo negro, el modo oscuro debería
aun así habilitarse automáticamente para que los comentarios sean legibles.

Sin embargo, el mecanismo de detección, que se basa en determinar la "luminancia", puede no activar el modo oscuro cuando lo desee. Para forzarlo, establezca la
bandera *hasDarkBackground* en true de la siguiente manera:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]