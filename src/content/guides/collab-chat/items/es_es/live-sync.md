### Actualizaciones en tiempo real

Collab Chat utiliza conexiones WebSocket para sincronizar todas las conversaciones en tiempo real entre todos los usuarios conectados. Cuando alguien crea una nueva anotación, añade un comentario o elimina una discusión, todos los demás usuarios que estén viendo la misma página ven la actualización de inmediato sin necesidad de recargar.

### Cómo funciona la sincronización por WebSocket

Cuando inicializas Collab Chat, el widget establece una conexión WebSocket con los servidores de FastComments. Esta conexión permanece abierta durante la sesión del usuario y escucha actualizaciones relacionadas con la página actual.

El sistema WebSocket utiliza tres tipos de mensajes de difusión para Collab Chat. El evento `new-text-chat` se activa cuando alguien crea una nueva anotación en la página. El evento `updated-text-chat` se activa cuando alguien actualiza una conversación existente. El evento `deleted-text-chat` se activa cuando alguien elimina una anotación.

### Sistema de ID de difusión

Para evitar efectos de eco en los que los usuarios ven sus propias acciones retransmitidas de vuelta, cada actualización incluye un `broadcastId` único. Cuando un usuario crea o actualiza una anotación, su cliente genera un UUID para esa operación. Cuando el WebSocket retransmite la actualización a todos los clientes, el cliente de origen ignora la actualización porque coincide con su propio `broadcastId`.

Esto garantiza una interacción fluida en la que los usuarios ven sus cambios de inmediato en la interfaz sin esperar el viaje de ida y vuelta al servidor, al mismo tiempo que asegura que todos los demás usuarios reciban la actualización.

### Conteo de usuarios en vivo

La barra superior muestra el número de usuarios que actualmente están viendo la página. Este conteo se actualiza en tiempo real a medida que los usuarios se unen y abandonan. El recuento de usuarios se proporciona a través de la misma conexión WebSocket y se incrementa/decrementa automáticamente en función de los eventos de conexión y desconexión.

### Resiliencia de la conexión

Si la conexión WebSocket se cae debido a problemas de red o mantenimiento del servidor, el widget intenta reconectarse automáticamente. Durante el período de reconexión, los usuarios aún pueden interactuar con las anotaciones existentes, pero no verán actualizaciones en tiempo real de otros usuarios hasta que la conexión se restablezca.

Una vez reconectado, el widget se resincroniza para asegurarse de que no se hayan perdido actualizaciones. Esto sucede de forma transparente sin requerir intervención del usuario.

### Consideraciones de ancho de banda

Los mensajes WebSocket son ligeros y contienen solo la información esencial necesaria para sincronizar el estado. Crear una nueva anotación normalmente usa menos de 1 KB de ancho de banda. El sistema también incluye agrupamiento inteligente para reducir la frecuencia de mensajes durante períodos de alta actividad.

Tus métricas de uso en el panel de FastComments rastrean `pubSubMessageCount` y `pubSubBandwidth` para que puedas supervisar la actividad de sincronización en tiempo real en tus sitios.

### Sincronización entre pestañas

Si un usuario tiene la misma página abierta en varias pestañas del navegador, las actualizaciones en una pestaña aparecen inmediatamente en las otras pestañas. Esto funciona mediante el mismo mecanismo de sincronización WebSocket y no requiere ninguna configuración adicional.

### Seguridad

Los mensajes WebSocket se transmiten a través de conexiones seguras (WSS) e incluyen validación del tenant para garantizar que los usuarios solo reciban actualizaciones de las conversaciones que están autorizados a ver. El servidor valida todas las operaciones antes de difundirlas para evitar acceso no autorizado o manipulación.