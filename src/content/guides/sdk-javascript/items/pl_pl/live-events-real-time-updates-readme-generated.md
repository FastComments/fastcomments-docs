Subskrybuj zdarzenia na żywo, aby otrzymywać aktualizacje w czasie rzeczywistym dotyczące komentarzy, głosów i innych aktywności.

### Zdarzenia na poziomie strony

Nasłuchuj zdarzeń na żywo dla konkretnej strony (komentarze, głosy itp.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Subskrybuj zdarzenia na żywo dla strony
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (pobierz to z odpowiedzi getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Zaktualizuj interfejs użytkownika o nowy komentarz
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Zaktualizuj licznik głosów w interfejsie użytkownika
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Zwróć true, jeśli zdarzenie zostało obsłużone
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Zamknij subskrypcję po zakończeniu
subscription.close();
```

### Subskrybuj zdarzenia użytkownika

Nasłuchuj zdarzeń specyficznych dla użytkownika (powiadomienia, wzmianki itp.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Pobierz to z odpowiedzi getComments
};

// Subskrybuj osobisty kanał użytkownika
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Pokaż powiadomienie w interfejsie użytkownika
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

// Zamknij po zakończeniu
userSubscription.close();
```

### Uzyskiwanie userIdWS

Parametr `userIdWS` jest wymagany do zdarzeń na żywo i można go uzyskać z odpowiedzi API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Wyodrębnij userIdWS z odpowiedzi
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Teraz możesz subskrybować zdarzenia na żywo
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```