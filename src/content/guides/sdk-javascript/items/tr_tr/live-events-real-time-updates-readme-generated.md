Yorumlar, oylar ve diğer etkinlikler için gerçek zamanlı güncellemeler almak üzere canlı etkinliklere abone olun.

### Sayfa Düzeyi Etkinlikleri

Belirli bir sayfadaki canlı etkinlikleri dinleyin (yorumlar, oylar, vb.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Belirli bir sayfa için canlı etkinliklere abone olun
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (bunu getComments yanıtından alın)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Yeni yorum ile UI'inizi güncelleyin
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Oy sayılarını arayüzünüzde güncelleyin
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

// Close the subscription when done
subscription.close();
```

### Kullanıcı Etkinliklerine Abone Olma

Kullanıcıya özel etkinlikleri dinleyin (bildirimler, bahsetmeler, vb.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Bu değeri getComments yanıtından alın
};

// Kullanıcının kişisel akışına abone olun
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Bildirimi arayüzünüzde gösterin
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

// İşiniz bittiğinde kapatın
userSubscription.close();
```

### userIdWS'i Alma

The `userIdWS` parameter is required for live events and can be obtained from API responses:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Yanıttan userIdWS'yi çıkarın
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Artık canlı etkinliklere abone olabilirsiniz
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```