תראה שעליך להעביר `broadcastId` בחלק מקריאות ה-API. כשאתה מקבל אירועים, תקבל חזרה את ה‑ID הזה, כך תדע להתעלם מהאירוע אם אתה מתכנן ליישם שינויים באופטימיות מצד הלקוח (מה שסביר שתרצה לעשות כיוון שזה מספק את חוויית המשתמש הטובה ביותר). העבר כאן UUID. ה‑ID צריך להיות ייחודי מספיק כדי שלא יופיע פעמיים באותו מושב בדפדפן.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // מזהה ייחודי לפעולה זו
  }
});
```