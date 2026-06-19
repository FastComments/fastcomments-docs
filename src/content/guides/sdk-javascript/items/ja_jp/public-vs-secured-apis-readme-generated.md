SDK は次の API クラスを提供します:

- **`DefaultApi`** - 認証に API キーを必要とする保護されたエンドポイント。サーバー側の操作に使用してください。
- **`PublicApi`** - API キーなしでアクセスできる公開エンドポイント。ブラウザやモバイル端末などから直接呼び出せます。
- **`ModerationApi`** - モデレーターダッシュボードのエンドポイント（コメントモデレーション、バン、バッジ、トラストファクター、検索）。モデレーターのセッションで認証されます。SSO 認証されたモデレーターの場合は `sso` クエリパラメータを渡してください。
- **`HiddenApi`** - 高度なユースケース向けの内部／管理者用エンドポイント。

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// ページのコメントを取得（APIキー不要）
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // この値は秘密にしてください！
});
const defaultApi = new DefaultApi(config);

// 管理者権限でコメントを取得
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath など */ });

// モデレーター認証済みの呼び出し（セッションクッキー、またはSSO認証されたモデレーターの場合は `sso` を渡してください）。
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```