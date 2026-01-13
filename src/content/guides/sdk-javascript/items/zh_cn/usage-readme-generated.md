该 SDK 为浏览器和服务器环境提供了不同的入口点，以确保最佳的兼容性和安全性：

### 浏览器 使用（客户端）

对于浏览器/前端应用，请使用不包含 Node.js 依赖项的浏览器安全导出：

```typescript
// 浏览器安全导入（不包含 Node.js 依赖）
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// 创建浏览器 SDK 实例
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // 可选，默认为 https://fastcomments.com
});

// 使用公共 API（无需 API 密钥 — 对浏览器安全）
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 服务器 使用（Node.js）

对于服务器/后端应用，请使用包含 SSO 和身份验证功能的完整 SDK：

```typescript
// 服务器端导入（包含 SSO，设计为与 NodeJS 一起使用）
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// 创建服务器 SDK 实例
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // 在服务器上保密！
  basePath: 'https://fastcomments.com' // 可选，默认为 https://fastcomments.com
});

// 使用带 API 密钥的安全 API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 仅类型导入

如果您只需要 TypeScript 类型（无运行时代码），请使用默认导入：

```typescript
// 仅类型（无运行时依赖 —— 在任何环境均安全）
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### 使用单个 API 类

#### 浏览器环境

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### 服务器环境  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```