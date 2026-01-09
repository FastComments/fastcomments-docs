The SDK provides three main API classes:

- **`DefaultApi`** - 受保护的端点，要求使用 API 密钥进行身份验证。用于服务器端操作。
- **`PublicApi`** - 公共端点，可在无需 API 密钥的情况下访问。可以直接从浏览器/移动设备等调用。
- **`HiddenApi`** - 用于高级用例的内部/管理端点。

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// 获取页面的评论（不需要 API 密钥）
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 请保密！
});
const defaultApi = new DefaultApi(config);

// 使用完整的管理员访问权限获取评论
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```