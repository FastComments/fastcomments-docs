SDKは以下のAPIクラスを提供します:

- **`DefaultApi`** - 認証にAPIキーが必要なセキュアなエンドポイントです。サーバーサイドの操作に使用します。
- **`PublicApi`** - APIキーなしでアクセスできる公開エンドポイントです。ブラウザやモバイルデバイスなどから直接呼び出すことができます。
- **`ModerationApi`** - モデレーターダッシュボードのエンドポイントです(コメントのモデレーション、BAN、バッジ、トラストファクター、検索)。モデレーターのセッションで認証されます。SSO認証されたモデレーターの場合は `sso` クエリパラメーターを渡してください。
- **`HiddenApi`** - 高度なユースケース向けの内部/管理者エンドポイントです。

### 例: Public APIの使用(ブラウザ対応)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Get comments for a page (no API key required)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 例: Default APIの使用(サーバーサイドのみ)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Keep this secret!
});
const defaultApi = new DefaultApi(config);

// Get comments with full admin access
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 例: Moderation APIの使用

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Moderator-authenticated calls (session cookie, or pass `sso` for an SSO moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```