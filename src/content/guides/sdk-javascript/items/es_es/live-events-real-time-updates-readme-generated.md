Suscríbete a eventos en vivo para recibir actualizaciones en tiempo real sobre comentarios, votos y otras actividades.

### Eventos a nivel de página

Escucha eventos en vivo en una página específica (comentarios, votos, etc.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Suscribirse a eventos en vivo para una página
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (obtén esto de la respuesta de getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Actualiza tu UI con el nuevo comentario
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Actualiza los contadores de votos en tu UI
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Devuelve true si el evento fue procesado
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Cierra la suscripción cuando hayas terminado
subscription.close();
```

### Suscribirse a eventos de usuario

Escucha eventos específicos del usuario (notificaciones, menciones, etc.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Obtén esto de la respuesta de getComments
};

// Suscribirse al feed personal del usuario
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Muestra la notificación en tu UI
        break;
      case LiveEventType.notification_update:
        console.log('Notification updated:', event.notification);
        break;
      default:
        console.log('Other user event:', event.type);
    }
    
    return true;
  },
  (isConnected: boolean) => {
    console.log('User feed connection:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Cierra cuando hayas terminado
userSubscription.close();
```

### Obtener userIdWS

El parámetro `userIdWS` es requerido para eventos en vivo y puede obtenerse de las respuestas de la API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Extrae userIdWS de la respuesta
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Ahora puedes suscribirte a eventos en vivo
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```