The SDK מספק שלוש מחלקות API עיקריות:

- **`DefaultApi`** - נקודות קצה מאובטחות שדורשות את מפתח ה-API שלך לאימות. השתמש בהן עבור פעולות בצד השרת.
- **`PublicApi`** - נקודות קצה ציבוריות שניתן לגשת אליהן ללא מפתח API. ניתן לקרוא להן ישירות מדפדפנים/מכשירים ניידים/וכו'.
- **`HiddenApi`** - נקודות קצה פנימיות/מנהליות לשימושים מתקדמים.

### דוגמה: שימוש ב-Public API (בטוח לדפדפן)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// קבל תגובות עבור דף (אין צורך במפתח API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### דוגמה: שימוש ב-Default API (צד שרת בלבד)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // שמור זאת בסוד!
});
const defaultApi = new DefaultApi(config);

// קבל תגובות עם גישה מנהלית מלאה
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```