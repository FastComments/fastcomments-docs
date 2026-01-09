댓글, 투표 및 기타 활동에 대한 실시간 업데이트를 받기 위해 라이브 이벤트를 구독하세요.

### 페이지 수준 이벤트

특정 페이지의 라이브 이벤트(댓글, 투표 등)를 수신합니다:

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// 특정 페이지의 라이브 이벤트를 구독합니다
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (getComments 응답에서 가져옵니다)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // 새 댓글로 UI를 업데이트합니다
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // UI의 투표 수를 업데이트합니다
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

사용자별 이벤트(알림, 멘션 등)를 수신합니다:

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // getComments 응답에서 가져옵니다
};

// 사용자의 개인 피드를 구독합니다
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // UI에 알림을 표시합니다
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

The `userIdWS` parameter is required for live events and can be obtained from API responses:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// 응답에서 userIdWS를 추출합니다
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // 이제 라이브 이벤트를 구독할 수 있습니다
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```