Subscribe to live events to get real-time updates for comments, votes, and other activities.

### Γεγονότα σε επίπεδο σελίδας

Ακούστε για ζωντανά γεγονότα σε μια συγκεκριμένη σελίδα (σχόλια, ψήφοι, κ.λπ.):

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// Εγγραφή σε ζωντανά γεγονότα για μια σελίδα
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (πάρτε αυτό από την απόκριση του getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Ενημερώστε το UI σας με το νέο σχόλιο
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Ενημερώστε τους μετρητές ψήφων στο UI σας
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Επιστρέψτε true αν το γεγονός χειρίστηκε
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Close the subscription when done
subscription.close();
```

### Εγγραφή σε γεγονότα χρήστη

Ακούστε για γεγονότα που αφορούν συγκεκριμένο χρήστη (ειδοποιήσεις, αναφορές, κ.λπ.):

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Πάρτε αυτό από την απάντηση του getComments
};

// Εγγραφή στο προσωπικό feed του χρήστη
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Εμφανίστε την ειδοποίηση στο UI σας
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

// Close when done
userSubscription.close();
```

### Λήψη userIdWS

The `userIdWS` parameter is required for live events and can be obtained from API responses:

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Εξαγάγετε το userIdWS από την απάντηση
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Τώρα μπορείτε να εγγραφείτε σε ζωντανά γεγονότα
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```