此 SDK 提供三個主要的 API 類別：

- **`DefaultApi`** - 需要您的 API 金鑰進行驗證的受保護端點。用於伺服器端操作。
- **`PublicApi`** - 可在無需 API 金鑰的情況下存取的公開端點。可直接從瀏覽器/行動裝置等呼叫。
- **`HiddenApi`** - 供內部/管理用途的端點，適用於進階使用情境。

### 範例：使用 Public API（瀏覽器安全）

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// 取得頁面評論（不需要 API 金鑰）
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 範例：使用 Default API（僅限伺服器端）

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 請保密！
});
const defaultApi = new DefaultApi(config);

// 以完整管理權限取得評論
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```