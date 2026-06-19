该 SDK 提供以下 API 类：

- **`DefaultApi`** - 需要使用 API 密钥进行身份验证的受保护端点。用于服务器端操作。
- **`PublicApi`** - 无需 API 密钥即可访问的公共端点。可直接从浏览器/移动设备等调用。
- **`ModerationApi`** - 管理员仪表板端点（评论审核、封禁、徽章、信任因子、搜索）。通过管理员的会话进行认证；对于通过 SSO 验证的管理员，可传递 `sso` 查询参数。
- **`HiddenApi`** - 用于高级用例的内部/管理员端点。

### 示例：使用 Public API（浏览器安全）

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Get comments for a page (no API key required)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 示例：使用 Default API（仅限服务器端）

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 请保密！
});
const defaultApi = new DefaultApi(config);

// Get comments with full admin access
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 示例：使用 Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath 等等 */ });

// Moderator-authenticated calls (session cookie, or pass `sso` for an SSO moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```