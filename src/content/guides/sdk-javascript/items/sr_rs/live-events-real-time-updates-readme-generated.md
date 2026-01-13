Pretplatite se na događaje uživo da biste dobili ažuriranja u realnom vremenu za komentare, glasove i druge aktivnosti.

### Događaji na nivou stranice

Slušajte događaje uživo na određenoj stranici (komentari, glasovi, itd.):

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
  'user-session-id', // userIdWS (dohvatite ovo iz odgovora getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ažurirajte vaš korisnički interfejs sa novim komentarom
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ažurirajte broj glasova u vašem korisničkom interfejsu
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Vrati true ako je događaj obrađen
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Zatvorite pretplatu kada završite
subscription.close();
```

### Pretplata na događaje korisnika

Slušajte događaje specifične za korisnika (obaveštenja, pominjanja, itd.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Dohvatite ovo iz odgovora getComments
};

// Pretplatite se na lični feed korisnika
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Prikažite obaveštenje u vašem korisničkom interfejsu
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

### Kako dobiti userIdWS

Parametar `userIdWS` je obavezan za događaje uživo i može se dobiti iz odgovora API-ja:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Izvucite userIdWS iz odgovora
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Sada se možete pretplatiti na događaje uživo
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```