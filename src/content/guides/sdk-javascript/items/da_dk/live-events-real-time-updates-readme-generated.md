Abonner på live-begivenheder for at få opdateringer i realtid om kommentarer, stemmer og andre aktiviteter.

### Begivenheder på sideniveau

Lyt efter live-begivenheder på en specifik side (kommentarer, stemmer osv.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Abonner på live-begivenheder for en side
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (hent dette fra getComments-responsen)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Opdater din UI med den nye kommentar
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Opdater stemmetællinger i din UI
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Returner true hvis begivenheden blev håndteret
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Luk abonnementet når det er færdigt
subscription.close();
```

### Abonner på brugerbegivenheder

Lyt efter brugerspecifikke begivenheder (notifikationer, nævnelser osv.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Hent dette fra getComments-responsen
};

// Abonner på brugerens personlige feed
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Vis notifikation i din UI
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

// Luk når du er færdig
userSubscription.close();
```

### Sådan får du userIdWS

Parameteren `userIdWS` er påkrævet for live-begivenheder og kan hentes fra API-responser:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Udtræk userIdWS fra responsen
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Nu kan du abonnere på live-begivenheder
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```