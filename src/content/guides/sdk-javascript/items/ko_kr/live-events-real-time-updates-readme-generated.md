실시간 이벤트를 구독하여 댓글, 투표 및 기타 활동에 대한 실시간 업데이트를 받아보세요.

### 페이지 수준 이벤트

특정 페이지에서 실시간 이벤트(댓글, 투표 등)를 수신하려면:

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// 페이지에 대한 실시간 이벤트 구독
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (getComments 응답에서 가져오세요)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // 새 댓글로 UI를 업데이트하세요
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // UI에서 투표 수를 업데이트하세요
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // 이벤트가 처리되었으면 true를 반환합니다
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// 완료되면 구독을 닫습니다
subscription.close();
```

### 사용자 이벤트 구독

사용자별 이벤트(알림, 멘션 등)를 수신하려면:

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // getComments 응답에서 가져오세요
};

// 사용자의 개인 피드 구독
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // UI에 알림을 표시하세요
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

// 완료되면 닫습니다
userSubscription.close();
```

### userIdWS 가져오기

`userIdWS` 매개변수는 실시간 이벤트에 필요하며 API 응답에서 얻을 수 있습니다:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// 응답에서 userIdWS 추출
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // 이제 실시간 이벤트를 구독할 수 있습니다
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```