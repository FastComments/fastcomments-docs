FastComments υποστηρίζει SSO για ενσωμάτωση με το υπάρχον σύστημα πιστοποίησης χρηστών σας. **Η λειτουργία SSO είναι διαθέσιμη μόνο στην έκδοση για διακομιστή** καθώς απαιτεί δυνατότητες κρυπτογράφησης του Node.js.

### Απλό SSO (Μόνο στην πλευρά διακομιστή)

Το απλό SSO θα πρέπει να δημιουργείται στην πλευρά διακομιστή και να αποστέλλεται στον πελάτη:

```typescript
// Κώδικας πλευράς διακομιστή (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Δημιουργία απλού SSO χρησιμοποιώντας τον ενσωματωμένο βοηθό  
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

// Στείλτε το ssoToken στον κώδικα της πλευράς πελάτη σας
// Ο κώδικας της πλευράς πελάτη μπορεί στη συνέχεια να χρησιμοποιήσει αυτό το token με το browser SDK
```

### Ασφαλές SSO (Πλευρά Διακομιστή, Συνιστάται)

Το ασφαλές SSO θα πρέπει να υλοποιείται στην πλευρά διακομιστή και παρέχει καλύτερη ασφάλεια:

```typescript
// Κώδικας πλευράς διακομιστή (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Δημιουργία ασφαλούς SSO χρησιμοποιώντας τον ενσωματωμένο βοηθό
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

// Χρήση με κλήσεις API στον διακομιστή
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ή στείλτε το ssoConfig στον πελάτη για χρήση στον browser
```

### Χρήση SSO από τον Browser (με Token που δημιουργείται από τον διακομιστή)

```typescript
// Κώδικας πλευράς πελάτη (browser)
import { PublicApi } from 'fastcomments-sdk/browser';

// Λάβετε το SSO token από το endpoint του διακομιστή σας
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
// Πλευρά διακομιστή: Δημιουργία SSO και σχολίου
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