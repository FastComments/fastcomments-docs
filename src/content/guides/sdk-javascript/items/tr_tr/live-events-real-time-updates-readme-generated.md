Yorumlar, oylar ve diğer etkinlikler için gerçek zamanlı güncellemeler almak üzere canlı etkinliklere abone olun.

### Sayfa Düzeyinde Etkinlikler

Belirli bir sayfada canlı etkinlikleri dinleyin (yorumlar, oylar vb.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Bir sayfa için canlı etkinliklere abone ol
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
        // Yeni yorumu arayüzünüzde güncelleyin
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
    
    return true; // Etkinlik işlendiyse true döndürün
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// İşiniz bittiğinde aboneliği kapatın
subscription.close();
```

### Kullanıcı Etkinliklerine Abone Olma

Kullanıcıya özel etkinlikleri dinleyin (bildirimler, bahsetmeler vb.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Bunu getComments yanıtından alın
};

// Kullanıcının kişisel akışına abone ol
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

### userIdWS Alma

`userIdWS` parametresi canlı etkinlikler için gereklidir ve API yanıtlarından elde edilebilir:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// userIdWS'yi yanıttan çıkarın
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Artık canlı etkinliklere abone olabilirsiniz
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```