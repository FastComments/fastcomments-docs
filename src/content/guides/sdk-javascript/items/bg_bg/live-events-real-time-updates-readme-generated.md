Абонирайте се за събития в реално време, за да получавате актуализации за коментари, гласове и други дейности.

### Събития на ниво страница

Слушайте за събития в реално време за конкретна страница (коментари, гласове и т.н.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Абонирайте се за събития в реално време за страница
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (получете това от отговора на getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Актуализирайте вашия интерфейс с новия коментар
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Актуализирайте броя на гласовете в потребителския интерфейс
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Върнете true, ако събитието е обработено
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Затворете абонамента, когато приключите
subscription.close();
```

### Абониране за потребителски събития

Слушайте за потребителски събития (известия, споменавания и т.н.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Получете това от отговора на getComments
};

// Абонирайте се за личния канал на потребителя
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Показване на известие в потребителския интерфейс
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

// Затворете, когато приключите
userSubscription.close();
```

### Получаване на userIdWS

Параметърът `userIdWS` е задължителен за събития в реално време и може да бъде получен от отговорите на API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Извлечете userIdWS от отговора
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Сега можете да се абонирате за събития в реално време
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```