該 SDK 提供以下 API 類別：

- **`DefaultApi`** - 需要您的 API 金鑰用於驗證的受保護端點。用於伺服器端作業。
- **`PublicApi`** - 可在未提供 API 金鑰下存取的公開端點。可直接從瀏覽器/行動裝置/等呼叫。
- **`ModerationApi`** - 管理員儀表板端點（留言審核、封鎖、徽章、信任指數、搜尋）。透過管理員的會話進行驗證；對於以 SSO 驗證的管理員，傳入 `sso` 查詢參數。
- **`HiddenApi`** - 用於進階使用情境的內部/管理端點。

### 範例：使用 Public API（適用於瀏覽器）

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// 取得頁面的留言（不需要 API 金鑰）
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 範例：使用 Default API（僅限伺服器端）

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 請務必保密！
});
const defaultApi = new DefaultApi(config);

// 以完整管理員權限取得留言
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 範例：使用 Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath 等 */ });

// 管理員已驗證的呼叫（使用 session cookie，或為 SSO 管理員傳入 `sso`）。
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```