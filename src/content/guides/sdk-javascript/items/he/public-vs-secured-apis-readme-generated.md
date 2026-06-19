ה-SDK מספק את מחלקות ה-API הבאות:

- **`DefaultApi`** - נקודות קצה מאובטחות שמחייבות את מפתח ה-API שלך לאימות. השתמש בהן עבור פעולות בצד השרת.
- **`PublicApi`** - נקודות קצה ציבוריות שניתן לגשת אליהן ללא מפתח API. ניתן לקרוא להן ישירות מדפדפנים/מכשירים ניידים/וכו'.
- **`ModerationApi`** - נקודות קצה של לוח המנהלים (ניהול תגובות, השעיות, תגים, גורם אמון, חיפוש). מאומתות על-ידי סשן המנהלים; העבר את פרמטר השאילתה `sso` עבור מנהלים המאומתים דרך SSO.
- **`HiddenApi`** - נקודות קצה פנימיות/מנהליות לשימושים מתקדמים.

### דוגמה: שימוש ב-API ציבורי (בטוח לדפדפן)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// קבל תגובות לעמוד (לא נדרש מפתח API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### דוגמה: שימוש ב-Default API (רק בצד השרת)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // שמור על זה בסוד!
});
const defaultApi = new DefaultApi(config);

// קבל תגובות עם גישת מנהל מלאה
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### דוגמה: שימוש ב-Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, וכו'. */ });

// קריאות מאומתות של מנהל (עוגיית סשן, או העבר את הפרמטר `sso` עבור מנהל המאומת דרך SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```