Iscriviti agli eventi live per ricevere aggiornamenti in tempo reale su commenti, voti e altre attività.

### Eventi a Livello di Pagina

Ascolta gli eventi live su una pagina specifica (commenti, voti, ecc.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Iscriviti agli eventi live per una pagina
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (ottienilo dalla risposta di getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Aggiorna la tua UI con il nuovo commento
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Aggiorna i conteggi dei voti nella tua UI
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Restituisci true se l'evento è stato gestito
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Chiudi la sottoscrizione quando hai finito
subscription.close();
```

### Iscriversi agli Eventi Utente

Ascolta gli eventi specifici dell'utente (notifiche, menzioni, ecc.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Ottienilo dalla risposta di getComments
};

// Iscriviti al feed personale dell'utente
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Mostra la notifica nella tua UI
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

// Chiudi quando hai finito
userSubscription.close();
```

### Ottenere userIdWS

Il parametro `userIdWS` è necessario per gli eventi live e può essere ottenuto dalle risposte API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Estrai userIdWS dalla risposta
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Ora puoi iscriverti agli eventi live
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```