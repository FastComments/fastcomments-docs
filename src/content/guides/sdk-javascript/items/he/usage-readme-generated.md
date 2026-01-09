חבילת SDK זו מספקת נקודות כניסה נפרדות לסביבת הדפדפן ולסביבת השרת על מנת להבטיח תאימות ואבטחה מיטבית:

### שימוש בדפדפן (צד-לקוח)

For browser/frontend applications, use the browser-safe export that excludes Node.js dependencies:

```typescript
// ייבוא בטוח לדפדפן (ללא תלות ב-Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// יצירת מופע SDK לדפדפן
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// השתמש ב-APIs ציבוריים (אין צורך במפתח API - בטוח לדפדפנים)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### שימוש בצד השרת (Node.js)

For server/backend applications, use the full SDK with SSO and authentication features:

```typescript
// ייבוא בצד השרת (כולל SSO ומתוכנן לעבוד עם NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// יצירת מופע SDK לשרת
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // שמרו על זה בסוד בשרת!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// השתמשו ב-APIs מאובטחות עם מפתח ה-API שלכם
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### ייבוא טיפוסים בלבד

If you only need TypeScript types (no runtime code), use the default import:

```typescript
// רק טיפוסים (ללא תלות בזמן ריצה - בטוח בכל מקום)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### שימוש במחלקות API בודדות

#### סביבה בדפדפן

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### סביבה בצד השרת  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```