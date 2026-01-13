Підписуйтеся на події в реальному часі, щоб отримувати оновлення щодо коментарів, голосів та інших дій.

### Події на рівні сторінки

Слухайте події в реальному часі для конкретної сторінки (коментарі, голоси тощо):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Підписатися на живі події для сторінки
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (отримайте це з відповіді getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Оновіть ваш інтерфейс користувача новим коментарем
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Оновіть підрахунок голосів у вашому інтерфейсі
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Поверніть true, якщо подію оброблено
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Close the subscription when done
subscription.close();
```

### Підписка на події користувача

Слухайте події, специфічні для користувача (сповіщення, згадки тощо):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Отримайте це з відповіді getComments
};

// Підписка на персональну стрічку користувача
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Показати сповіщення у вашому інтерфейсі
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

// Закрийте, коли закінчите
userSubscription.close();
```

### Отримання userIdWS

Параметр `userIdWS` потрібен для живих подій і може бути отриманий з відповідей API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Витягніть userIdWS з відповіді
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Тепер ви можете підписатися на живі події
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```