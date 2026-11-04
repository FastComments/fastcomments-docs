ה-SDK הזה מספק נקודות כניסה נפרדות עבור סביבות דפדפן ושרת כדי להבטיח תאימות ואבטחה אופטימליות:

### שימוש בדפדפן (צד לקוח)

עבור אפליקציות דפדפן/frontend, השתמשו בייצוא הבטוח לדפדפן שאינו כולל תלותי Node.js:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### שימוש בשרת (Node.js)

עבור אפליקציות שרת/backend, השתמשו ב-SDK המלא עם תכונות SSO ואימות:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### ייבוא של טיפוסים בלבד

אם אתם זקוקים רק לטיפוסי TypeScript (ללא קוד ריצה), השתמשו בייבוא ברירת המחדל:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### שימוש במחלקות API בודדות

#### סביבת דפדפן

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### סביבת שרת  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```