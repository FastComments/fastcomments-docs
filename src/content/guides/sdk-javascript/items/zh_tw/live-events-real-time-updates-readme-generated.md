訂閱即時事件以獲取有關留言、點讚和其他活動的即時更新。

### 頁面層級事件

監聽特定頁面的即時事件（留言、點讚等）：

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// 訂閱該頁面的即時事件
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS（租戶 ID）
  'page-url-id',    // urlIdWS（頁面 URL ID）  
  'user-session-id', // userIdWS（從 getComments 回應取得）
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // 使用新留言更新您的介面
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // 在介面中更新票數
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // 若事件已處理則回傳 true
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// 完成後關閉訂閱
subscription.close();
```

### 訂閱使用者事件

監聽與特定使用者相關的事件（通知、提及等）：

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // 從 getComments 回應取得
};

// 訂閱使用者的個人資訊流
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // 在您的介面中顯示通知
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

// 完成後關閉
userSubscription.close();
```

### 取得 userIdWS

參數 `userIdWS` 為即時事件所需，可從 API 回應中取得：

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// 從回應中擷取 userIdWS
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // 現在您可以訂閱即時事件
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```