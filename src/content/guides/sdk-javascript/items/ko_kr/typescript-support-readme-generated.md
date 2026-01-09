이 SDK는 TypeScript로 작성되었으며 모든 API 메서드와 응답 모델에 대한 완전한 타입 정의를 제공합니다:

```typescript
// 기본 내보내기에서 타입을 가져옵니다 (어디서나 안전함)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// 브라우저 SDK와 함께 사용
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```