このSDKはTypeScriptで記述されており、すべてのAPIメソッドとレスポンスモデルに対する完全な型定義を提供します:

```typescript
// デフォルトエクスポートから型をインポート（どこでも安全です）
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// ブラウザSDKで使用
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```