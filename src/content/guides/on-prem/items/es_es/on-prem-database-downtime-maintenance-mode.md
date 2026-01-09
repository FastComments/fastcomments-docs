FastComments admite un modo de mantenimiento automático. Si la base de datos deja de funcionar, puede seguir sirviendo los hilos de comentarios más populares.

Además, en modo de mantenimiento, todos los comentarios se guardan en `BACKUP_DIR`. Se procesarán (verificados por spam, etc.) y se guardarán una vez que el sistema vuelva a estar en línea.

Lo hace determinando, cada hora, los 100 hilos de comentarios más populares y almacenando en caché su contenido en disco. Determinar los 100 hilos
ya se realiza a partir de un estado precalculado, por lo que no es un trabajo periódico pesado.

Esto es completamente opcional, y solo se habilita si `CACHE_DIR` y `BACKUP_DIR` están establecidos. Esto, por supuesto, hace que los nodos de la aplicación sean con estado, sin embargo es un estado que
puede perderse en cualquier momento sin provocar que la aplicación se comporte mal.

Tenga en cuenta que en modo de mantenimiento no se puede realizar una autenticación adecuada de los hilos de comentarios, por lo que solo se respaldan periódicamente los hilos que se consideran de manera segura públicos.

En modo de mantenimiento muchas funciones no están disponibles.