Претплатите се на догађаје уживо да бисте добијали ажурирања у реалном времену за коментаре, гласове и друге активности.

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
  'user-session-id', // userIdWS (преузмите ово из одговора getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ажурирајте ваш кориснички интерфејс са новим коментаром
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ажурирајте број гласова у вашем корисничком интерфејсу
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Return true if event was handled
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Затворите претплату када завршите
subscription.close();
```

### Претплата на корисничке догађаје

Слушајте догађaje специфичне за корисника (обавештења, помињања итд.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Преузмите ово из одговора getComments
};

// Претплатите се на лични фид корисника
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Прикажите обавештење у вашем корисничком интерфејсу
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

// Извуците userIdWS из одговора
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Сада се можете претплатити на догађаје уживо
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```