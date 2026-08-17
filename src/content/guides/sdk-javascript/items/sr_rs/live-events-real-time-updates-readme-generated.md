Subscribe to live events to get real-time updates for comments, votes, and other activities.

### Догађаји на нивоу странице

Слушајте живе догађаје на одређеној страници (коментари, гласови, итд.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Пријавите се на живе догађаје за страницу
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (добавите ово из getComments одговора)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ажурирајте ваш UI новим коментаром
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ажурирајте број гласа у вашем UI-у
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Вратите true ако је догађај обрађен
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Затворите претплату када завршите
subscription.close();
```

### Пријава на корисничке догађаје

Слушајте догађаје специфичне за корисника (обавештења, помињањања, итд.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Добијте ово из getComments одговора
};

// Пријавите се на лични фид
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Прикажите обавештење у вашем UI-у
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

// Затворите када завршите
userSubscription.close();
```

### Добијање userIdWS

Параметар `userIdWS` је потребан за живе догађаје и може се добити из API одговора:

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