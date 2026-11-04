订阅实时事件以获取评论、投票和其他活动的实时更新。

### 页面级事件

监听特定页面的实时事件（评论、投票等）：

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// 订阅页面的实时事件
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS（从 getComments 响应中获取）
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // 用新评论更新你的 UI
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // 更新 UI 中的投票数
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // 如果事件已处理则返回 true
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// 完成后关闭订阅
subscription.close();
```

### 订阅用户事件

监听用户特定的事件（通知、提及等）：

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // 从 getComments 响应中获取
};

// 订阅用户的个人动态
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // 在你的 UI 中显示通知
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

// 完成后关闭
userSubscription.close();
```

### 获取 userIdWS

`userIdWS` 参数是实时事件所必需的，可以从 API 响应中获取：

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// 从响应中提取 userIdWS
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // 现在你可以订阅实时事件了
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```