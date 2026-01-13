### Actualizaciones en tiempo real

Image Chat utiliza conexiones WebSocket para sincronizar todas las conversaciones en tiempo real entre todos los usuarios conectados. Cuando alguien crea un nuevo marcador, añade un comentario o elimina una discusión, todos los demás usuarios que están viendo la misma imagen ven la actualización inmediatamente sin necesidad de refrescar.

### Cómo funciona la sincronización por WebSocket

Al inicializar Image Chat, el widget establece una conexión WebSocket con los servidores de FastComments. Esta conexión permanece abierta durante la sesión del usuario y escucha actualizaciones relacionadas con la imagen actual.

El sistema WebSocket utiliza tres tipos de mensajes de difusión para Image Chat. El evento `new-image-chat` se dispara cuando alguien crea un nuevo marcador en la imagen. El evento `image-chat-updated` se dispara cuando alguien actualiza una conversación existente. El evento `deleted-image-chat` se dispara cuando alguien elimina un marcador.

### Sistema de ID de difusión

Para evitar efectos de eco en los que los usuarios ven sus propias acciones retransmitidas, cada actualización incluye un `broadcastId` único. Cuando un usuario crea o actualiza un marcador, su cliente genera un UUID para esa operación. Cuando el WebSocket retransmite la actualización a todos los clientes, el cliente origen ignora la actualización porque coincide con su propio `broadcastId`.

Esto asegura una interacción fluida en la que los usuarios ven sus cambios inmediatamente en la interfaz sin esperar el viaje de ida y vuelta al servidor, al mismo tiempo que garantiza que todos los demás usuarios reciban la actualización.

### Resiliencia de la conexión

Si la conexión WebSocket se cae debido a problemas de red o mantenimiento del servidor, el widget intenta reconectarse automáticamente. Durante el periodo de reconexión, los usuarios aún pueden interactuar con los marcadores existentes, pero no verán las actualizaciones en tiempo real de otros usuarios hasta que la conexión se restablezca.

Una vez reconectado, el widget vuelve a sincronizarse para asegurar que no se hayan perdido actualizaciones. Esto ocurre de forma transparente sin requerir intervención del usuario.

### Consideraciones sobre el ancho de banda

Los mensajes WebSocket son ligeros y contienen solo la información esencial necesaria para sincronizar el estado. Crear un nuevo marcador normalmente utiliza menos de 1KB de ancho de banda. El sistema también incluye agrupamiento inteligente para reducir la frecuencia de mensajes durante periodos de alta actividad.

Las métricas de uso en el panel de FastComments rastrean `pubSubMessageCount` y `pubSubBandwidth` para que puedas monitorizar la actividad de sincronización en tiempo real en tus sitios.

### Sincronización entre pestañas

Si un usuario tiene la misma página abierta en varias pestañas del navegador, las actualizaciones en una pestaña aparecen inmediatamente en las otras pestañas. Esto funciona mediante el mismo mecanismo de sincronización por WebSocket y no requiere configuración adicional.

Los usuarios pueden tener tu sitio abierto en varios dispositivos simultáneamente, y todos ellos permanecerán sincronizados. Un marcador creado en un ordenador de sobremesa aparece al instante en la tableta del usuario si ambos dispositivos están viendo la misma imagen.

### Seguridad

Los mensajes WebSocket se transmiten sobre conexiones seguras (WSS) e incluyen validación de tenant para asegurar que los usuarios solo reciben actualizaciones de las conversaciones que están autorizados a ver. El servidor valida todas las operaciones antes de difundirlas para prevenir acceso o manipulación no autorizada.

### Comportamiento sin conexión

Cuando los usuarios están completamente sin conexión, aún pueden ver los marcadores existentes pero no pueden crear nuevos ni ver las actualizaciones de otros. El widget detecta el estado sin conexión y muestra un mensaje apropiado.

Si un usuario intenta crear un marcador estando sin conexión y luego vuelve a conectarse, la operación fallará en lugar de ponerse en cola, asegurando la consistencia de los datos. Los usuarios deben reintentar la operación una vez que su conexión se haya restablecido.

### Impacto en el rendimiento

La conexión WebSocket tiene un impacto mínimo en el rendimiento. La conexión permanece inactiva cuando no se producen actualizaciones y solo procesa mensajes cuando hay actividad. En una imagen típica con actividad moderada de marcadores, el WebSocket utiliza menos CPU que el renderizado de la propia imagen.

Para páginas con cientos de usuarios simultáneos y alta actividad de creación de marcadores, el sistema escala horizontalmente para mantener el rendimiento sin afectar las conexiones de clientes individuales.

### Casos de uso colaborativos

La sincronización en tiempo real hace que Image Chat sea especialmente potente para flujos de trabajo colaborativos. Los equipos de diseño pueden revisar maquetas juntos, con todos viendo la colocación de marcadores en tiempo real. Los equipos de soporte al cliente pueden anotar capturas de pantalla de forma colaborativa para identificar problemas. Los grupos educativos pueden discutir diagramas con todos los participantes viendo los marcadores de los demás a medida que se crean.

La retroalimentación inmediata crea una experiencia colaborativa más atractiva y productiva en comparación con los sistemas de comentarios tradicionales donde los usuarios necesitan refrescar para ver las actualizaciones.