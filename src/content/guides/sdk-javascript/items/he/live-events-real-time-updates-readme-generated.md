הירשם לאירועים חיים כדי לקבל עדכונים בזמן אמת על תגובות, הצבעות ופעילויות נוספות.

### אירועים ברמת הדף

האזן לאירועים חיים בדף מסוים (תגובות, הצבעות וכו׳):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// הירשם לאירועים חיים עבור דף
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (קבל זאת מתוך התגובה של getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // עדכן את ממשק המשתמש שלך עם התגובה החדשה
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // עדכן את ספירות ההצבעות בממשק המשתמש שלך
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // החזר true אם האירוע טופל
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// סגור את המנוי כשסיימת
subscription.close();
```

### הירשם לאירועי משתמש

האזן לאירועים ספציפיים למשתמש (התראות, אזכורים וכו׳):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Get this from getComments response
};

// הירשם לפיד האישי של המשתמש
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // הצג הודעה בממשק המשתמש שלך
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

// סגור כשסיימת
userSubscription.close();
```

### קבלת userIdWS

פרמטר `userIdWS` נדרש עבור אירועים חיים וניתן להשיגו מתשובות API:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// חלץ את userIdWS מהתגובה
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // כעת אתה יכול להירשם לאירועים חיים
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```