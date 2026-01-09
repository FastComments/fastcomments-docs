Pretplatite se na događaje uživo kako biste dobili ažuriranja u stvarnom vremenu za komentare, glasove i druge aktivnosti.

### Događaji na razini stranice

Slušajte događaje uživo na određenoj stranici (komentari, glasovi itd.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Pretplatite se na događaje uživo za stranicu
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (dobijte ovo iz getComments odgovora)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ažurirajte svoje korisničko sučelje s novim komentarom
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ažurirajte brojače glasova u vašem korisničkom sučelju
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Vratite true ako je događaj obrađen
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Close the subscription when done
subscription.close();
```

### Pretplata na korisničke događaje

Slušajte događaje specifične za korisnika (obavijesti, spominjanja itd.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Get this from getComments response
};

// Pretplatite se na osobni feed korisnika
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Prikažite obavijest u vašem korisničkom sučelju
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

// Zatvorite kada završite
userSubscription.close();
```

### Dobivanje userIdWS

Parametar `userIdWS` je obavezan za događaje uživo i može se dobiti iz odgovora API-ja:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Extract userIdWS from the response
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Now you can subscribe to live events
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```