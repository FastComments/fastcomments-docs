ה-SDK נכתב ב-TypeScript ומספק הגדרות טיפוס מלאות לכל שיטות ה-API ולמודלים של התשובות:

```typescript
// ייבא טיפוסים מהיצוא ברירת המחדל (בטוח בכל מקום)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// שימוש עם ה-SDK לדפדפן
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```