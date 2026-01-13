Подпишитесь на live-события, чтобы получать обновления в реальном времени о комментариях, голосах и других действиях.

### События на уровне страницы

Отслеживайте live-события на конкретной странице (комментарии, голоса и т.д.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Подписаться на live-события для страницы
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (получить из ответа getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Обновите интерфейс, добавив новый комментарий
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Обновите счетчики голосов в интерфейсе
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Вернуть true, если событие обработано
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Close the subscription when done
subscription.close();
```

### Подписка на пользовательские события

Отслеживайте события, связанные с пользователем (уведомления, упоминания и т.д.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Получите это из ответа getComments
};

// Подписаться на личную ленту пользователя
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Показать уведомление в интерфейсе
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

// Закрыть по завершении
userSubscription.close();
```

### Получение userIdWS

Параметр `userIdWS` требуется для live-событий и может быть получен из ответов API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Извлечь userIdWS из ответа
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Теперь можно подписаться на live-события
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```