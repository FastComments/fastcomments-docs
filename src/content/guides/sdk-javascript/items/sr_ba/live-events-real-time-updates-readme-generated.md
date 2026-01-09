Претплатите се на догађаје уживо да бисте добили ажурирања у реалном времену за коментаре, гласове и друге активности.

### Догађаји на нивоу странице

Слушајте догађаје уживо за одређену страницу (коментари, гласови итд.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Претплатите се на догађаје уживо за страницу
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (добијте ово из getComments одговора)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ажурирајте ваш кориснички интерфејс новим коментаром
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ажурирајте бројеве гласова у вашем корисничком интерфејсу
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

### Претплатите се на корисничке догађаје

Слушајте догађаје специфичне за корисника (нотификације, помена итд.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Узмите ово из getComments одговора
};

// Претплатите се на лични фид корисника
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Прикажите нотификацију у вашем корисничком интерфејсу
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

Параметар `userIdWS` је обавезан за догађаје уживо и може се добити из одговора API-ја:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Извучите userIdWS из одговора
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Сада се можете претплатити на догађаје уживо
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```