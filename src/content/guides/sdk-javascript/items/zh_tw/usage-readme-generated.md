此 SDK 為瀏覽器與伺服器環境提供獨立的進入點，以確保最佳相容性與安全性：

### 瀏覽器 用法（客戶端）

對於瀏覽器/前端應用，請使用不包含 Node.js 相依性的瀏覽器安全匯出：

```typescript
// 瀏覽器安全匯入（不含 Node.js 相依性）
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// 建立瀏覽器 SDK 實例
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // 可選，預設為 https://fastcomments.com
});

// 使用公開 API（不需 API 金鑰 - 對瀏覽器安全）
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 伺服器 用法（Node.js）

對於伺服器/後端應用，請使用包含 SSO 與驗證功能的完整 SDK：

```typescript
// 伺服器端匯入（包含 SSO，並設計為可在 NodeJS 環境中運行）
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// 建立伺服器 SDK 實例
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // 請在伺服器上保密！
  basePath: 'https://fastcomments.com' // 可選，預設為 https://fastcomments.com
});

// 使用需 API 金鑰的受保護 API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 僅匯入類型

如果您只需要 TypeScript 類型（沒有執行時程式碼），請使用預設匯入：

```typescript
// 僅類型（無執行時相依性 - 在任何地方皆安全）
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### 使用個別 API 類別

#### 瀏覽器 環境

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### 伺服器 環境  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```