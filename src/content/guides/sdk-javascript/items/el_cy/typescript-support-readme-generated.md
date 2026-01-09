Το SDK είναι γραμμένο σε TypeScript και παρέχει πλήρεις ορισμούς τύπων για όλες τις μεθόδους API και τα μοντέλα απάντησης:

```typescript
// Εισαγωγή τύπων από την προεπιλεγμένη εξαγωγή (ασφαλές παντού)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Χρήση με το browser SDK
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```