FastComments תומך ב-SSO כדי להשתלב במערכת אימות המשתמשים הקיימת שלך. **פונקציונליות SSO זמינה רק בגרסת השרת** מכיוון שהיא דורשת יכולות crypto של Node.js.

### SSO פשוט (צד שרת בלבד)

יש ליצור SSO פשוט בצד השרת ולשלוח אותו אל הלקוח:

```typescript
// קוד צד שרת (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// יצירת SSO פשוט באמצעות עזר מובנה  
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

// שלח את ssoToken לקוד בצד הלקוח
// קוד בצד הלקוח יכול אז להשתמש בטוקן זה עם ה-SDK לדפדפן
```

### SSO מאובטח (צד שרת, מומלץ)

יש ליישם SSO מאובטח בצד השרת והוא מספק אבטחה טובה יותר:

```typescript
// קוד צד שרת (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// יצירת SSO מאובטח באמצעות העזר המובנה
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

### שימוש ב-SSO מהדפדפן (עם טוקן שנוצר בשרת)

```typescript
// קוד בצד הלקוח (דפדפן)
import { PublicApi } from 'fastcomments-sdk/browser';

// קבל טוקן SSO מנקודת הקצה של השרת שלך
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // השתמש בטוקן SSO שנוצר בשרת
});
```

### SSO עם יצירת תגובה

```typescript
// צד שרת: צור SSO ותגובה
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