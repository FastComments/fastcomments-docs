FastComments υποστηρίζει SSO για να ενσωματωθεί με το υπάρχον σύστημα αυθεντικοποίησης χρηστών σας. **Η λειτουργία SSO είναι διαθέσιμη μόνο στο server export** καθώς απαιτεί δυνατότητες crypto του Node.js.

### Simple SSO (Server-Side Only)

Το απλό SSO πρέπει να δημιουργείται στο server και να αποστέλλεται στον client:

```typescript
// Κώδικας στο server (Node.js/backend)
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

### Secure SSO (Server-Side, Recommended)

Το Ασφαλές SSO πρέπει να υλοποιηθεί στο server και παρέχει καλύτερη ασφάλεια:

```typescript
// Κώδικας στο server (Node.js/backend)
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

### Using SSO from Browser (with Server-Generated Token)

Χρήση SSO από το πρόγραμμα περιήγησης (με token που δημιουργείται από τον server)

```typescript
// Κώδικας client-side (πρόγραμμα περιήγησης)
import { PublicApi } from 'fastcomments-sdk/browser';

// Get SSO token from your server endpoint
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Χρησιμοποιήστε το SSO token που δημιουργήθηκε από τον server
});
```

### SSO with Comment Creation

SSO με δημιουργία σχολίου

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