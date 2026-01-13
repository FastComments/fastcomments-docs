Naročite se na dogodke v živo, da dobite posodobitve v realnem času za komentarje, glasove in druge dejavnosti.

### Dogodki na ravni strani

Poslušajte dogodke v živo na določeni strani (komentarji, glasovi itd.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Naroči se na dogodke v živo za stran
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (pridobite ga iz odgovora getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Posodobite svoj uporabniški vmesnik z novim komentarjem
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Posodobite število glasov v vašem uporabniškem vmesniku
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Vrni true, če je bil dogodek obdelan
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Zaprite naročnino, ko končate
subscription.close();
```

### Naročite se na uporabniške dogodke

Poslušajte dogodke, ki so specifični za uporabnika (obvestila, omembe itd.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Pridobite to iz odgovora getComments
};

// Naročite se na osebni vir uporabnika
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Prikažite obvestilo v vašem uporabniškem vmesniku
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

// Zaprite, ko končate
userSubscription.close();
```

### Pridobivanje userIdWS

Parameter `userIdWS` je obvezen za dogodke v živo in ga je mogoče pridobiti iz odgovorov API-ja:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Izvlecite userIdWS iz odgovora
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Zdaj se lahko naročite na dogodke v živo
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```