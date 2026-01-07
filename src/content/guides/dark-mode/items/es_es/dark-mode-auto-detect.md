Por defecto, FastComments detectará automáticamente si su sitio tiene un fondo oscuro basándose en la "distancia del negro" en el círculo de colores.

Nuestros productos hacen lo mejor que pueden en esto, sin embargo, hay casi infinitos colores en la rueda de colores, y puede haber escenarios donde la aplicación elige usar el modo oscuro cuando no es apropiado, y viceversa. Esta documentación cubre cómo tener un control más detallado sobre esto.

#### Detalles técnicos

Detectamos el modo oscuro recorriendo los elementos de la página hacia arriba desde el widget de comentarios, buscando un fondo oscuro cuando el widget se carga inicialmente.

Para alternar el modo oscuro después de este paso, debe llamar al widget para actualizar su configuración. Esto se cubre en la sección `Configuración manual`.
