Abonneer op live-evenementen om realtime updates te ontvangen voor reacties, stemmen en andere activiteiten.

### Paginaniveau-evenementen

Luister naar live-evenementen op een specifieke pagina (reacties, stemmen, enz.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Abonneer op live-evenementen voor een pagina
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (haal dit uit het antwoord van getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Werk je UI bij met de nieuwe reactie
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Werk de stemaantallen in je UI bij
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Geef true terug als het event is verwerkt
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Sluit de subscription wanneer klaar
subscription.close();
```

### Abonneer op gebruikersgebeurtenissen

Luister naar gebruiker-specifieke gebeurtenissen (meldingen, vermeldingen, enz.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Verkrijg dit uit het antwoord van getComments
};

// Abonneer op de persoonlijke feed van de gebruiker
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Toon melding in je UI
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

// Sluit wanneer klaar
userSubscription.close();
```

### Het verkrijgen van userIdWS

De `userIdWS`-parameter is vereist voor live-evenementen en kan worden verkregen uit API-responses:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Extraheer userIdWS uit het antwoord
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Nu kun je je abonneren op live-evenementen
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```