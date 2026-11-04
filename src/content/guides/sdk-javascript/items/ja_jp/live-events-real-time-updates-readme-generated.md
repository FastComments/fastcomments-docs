ライブイベントを購読すると、コメント、投票、その他のアクティビティのリアルタイム更新を受け取ることができます。

### ページレベルのイベント

特定のページのライブイベント(コメント、投票など)を購読します:

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// ページのライブイベントを購読する
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (getComments レスポンスから取得します)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // 新しいコメントでUIを更新する
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // UI内の投票数を更新する
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // イベントが処理された場合は true を返す
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// 完了したら購読を閉じる
subscription.close();
```

### ユーザーイベントの購読

ユーザー固有のイベント(通知、メンションなど)を購読します:

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // getComments レスポンスから取得します
};

// ユーザーの個人フィードを購読する
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // UIに通知を表示する
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

// 完了したら閉じる
userSubscription.close();
```

### userIdWS の取得

`userIdWS` パラメータはライブイベントに必要で、APIレスポンスから取得できます:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// レスポンスから userIdWS を抽出する
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // これでライブイベントを購読できます
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```