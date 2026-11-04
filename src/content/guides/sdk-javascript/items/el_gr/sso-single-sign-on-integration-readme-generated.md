Το FastComments υποστηρίζει SSO για ενσωμάτωση με το υπάρχον σύστημα ελέγχου ταυτότητας χρηστών σας. **Η λειτουργία SSO είναι διαθέσιμη μόνο στο server export** καθώς απαιτεί δυνατότητες κρυπτογραφίας του Node.js.

### Απλό SSO (Μόνο Server-Side)

Το απλό SSO πρέπει να δημιουργείται server-side και να αποστέλλεται στον client:

```typescript
// Server-side code (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create simple SSO using the built-in helper  
const userData = {
  username: 'john_doe',
  email: 'john@example.com',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg'
};

const sso = FastCommentsSSO.createSimple(userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoToken = sso.createToken();

// Send ssoToken to your client-side code
// Client-side code can then use this token with the browser SDK
```

### Ασφαλές SSO (Server-Side, Προτεινόμενο)

Το ασφαλές SSO πρέπει να υλοποιείται server-side και παρέχει καλύτερη ασφάλεια:

```typescript
// Server-side code (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create secure SSO using the built-in helper
const userData = {
  id: 'user-123',
  email: 'john@example.com',
  username: 'john_doe',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg',
  isAdmin: false,
  isModerator: false
};

const sso = FastCommentsSSO.createSecure('your-api-key', userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoConfig = sso.prepareToSend();

// Use with API calls on the server
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Or send ssoConfig to client for browser usage
```

### Χρήση SSO από τον Browser (με Token που Δημιουργείται από τον Server)

```typescript
// Client-side code (browser)
import { PublicApi } from 'fastcomments-sdk/browser';

// Get SSO token from your server endpoint
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO με Δημιουργία Σχολίου

```typescript
// Server-side: Create SSO and comment
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

const sso = FastCommentsSSO.createSecure('your-api-key', userData);
const ssoConfig = sso.prepareToSend();

const response = await publicApi.createCommentPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  broadcastId: 'unique-broadcast-id',
  commentData: {
    comment: 'This is my comment',
    date: Date.now(),
    commenterName: 'John Doe',
    url: 'https://example.com/page',
    urlId: 'page-url-id'
  },
  sso: JSON.stringify(ssoConfig)
});
```