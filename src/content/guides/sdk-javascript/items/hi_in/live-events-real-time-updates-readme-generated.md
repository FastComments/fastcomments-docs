कमेंट्स, वोट्स और अन्य गतिविधियों के लिए रीयल-टाइम अपडेट प्राप्त करने के लिए लाइव इवेंट्स की सदस्यता लें।

### पेज-स्तरीय इवेंट्स

किसी विशिष्ट पेज पर लाइव इवेंट्स सुनें (कमेंट्स, वोट्स, आदि):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// किसी पेज के लिए लाइव इवेंट्स की सदस्यता लें
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (यह getComments रिस्पॉन्स से प्राप्त करें)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // नए कमेंट के साथ अपनी UI अपडेट करें
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // अपनी UI में वोट काउंट अपडेट करें
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // यदि इवेंट हैंडल किया गया तो true रिटर्न करें
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// काम पूरा होने पर सदस्यता बंद करें
subscription.close();
```

### यूज़र इवेंट्स की सदस्यता लें

यूज़र-विशिष्ट इवेंट्स सुनें (नोटिफिकेशन, मेंशन्स, आदि):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // यह getComments रिस्पॉन्स से प्राप्त करें
};

// यूज़र की व्यक्तिगत फ़ीड की सदस्यता लें
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // अपनी UI में नोटिफिकेशन दिखाएँ
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

// काम पूरा होने पर बंद करें
userSubscription.close();
```

### userIdWS प्राप्त करना

लाइव इवेंट्स के लिए `userIdWS` पैरामीटर आवश्यक है और इसे API रिस्पॉन्स से प्राप्त किया जा सकता है:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// रिस्पॉन्स से userIdWS निकालें
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // अब आप लाइव इवेंट्स की सदस्यता ले सकते हैं
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```