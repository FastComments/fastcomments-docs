Usar FastComments Comentarios Programados es simple. Primero, querras [importar los comentarios aqui](https://fastcomments.com/auth/my-account/manage-data/import-scheduled).

Esta pagina tambien se puede acceder a traves de `Manage Data->Schedule Comments`.

### Comentarios reproducidos manualmente

*Para comentarios* reproducidos manualmente (tienes que presionar Play manualmente), tienes la opcion de comenzar a reproducir comentarios. Esto reproducira los comentarios en cada pagina que hayas definido en el archivo CSV,
con los retrasos entre cada comentario basados en el retraso que especificaste.

Esto es util cuando tienes un webinar en vivo programado que comienza a una hora especifica. Cuando el webinar comience, simplemente presiona Play
en el panel de control.

### Reproduccion automatica de comentarios

*Para comentarios* reproducidos automaticamente, los comentarios se reproducen en cada carga de pagina para cada usuario.

Esto es util para escenarios donde videos u otro contenido comienza desde el principio con cada carga.

### Reproduccion con crecimiento dinamico

Cada vez que el script de reproduccion automatica se ejecuta para un usuario - al cargar la pagina - todavia existe
la oportunidad para que otros comenten.

A medida que las personas dejan comentarios, sus comentarios se **agregan automaticamente al script de
reproduccion** con el mismo desplazamiento desde la carga de la pagina, por lo que la conversacion continua creciendo sin
trabajo manual.

Puedes adicionalmente **moderar** los comentarios publicados, para seleccionar que comentarios quieres mostrar
cada vez que se ejecuta el script de reproduccion automatica.

La pagina `Moderate Comments` tambien mostrara una marca de tiempo como `AutoPlay 1hr 2m 30s` junto a cada
comentario en lugar de la fecha.

Esto solo esta disponible para reproduccion automatica, no para reproduccion programada manualmente.

### Configuracion

Cada comentario se publicara **en vivo**. Puedes considerar [activar mostrar comentarios en vivo de inmediato](/guide-customizations-and-configuration.html#show-live-right-away).

Puedes aprender sobre el formato de importacion en la seccion Formato de Importacion de esta documentacion.
