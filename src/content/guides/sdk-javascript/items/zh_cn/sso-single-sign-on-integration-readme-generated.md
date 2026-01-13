FastComments 支持 SSO，以与您现有的用户认证系统集成。 **SSO 功能仅在服务器导出中可用**，因为它需要 Node.js crypto 功能。

### 简单 SSO（仅服务器端）

简单 SSO 应在服务器端生成并发送到客户端：

```typescript
// 服务器端代码（Node.js/后端）
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 使用内置辅助创建简单 SSO  
const userData = {
  username: 'john_doe',
  email: 'john@example.com',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg'
};

const sso = FastCommentsSSO.createSimple(userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoToken = sso.createToken();

// 将 ssoToken 发送到您的客户端代码
// 客户端代码然后可以使用该令牌与浏览器 SDK 一起使用
```

### 安全 SSO（服务器端，推荐）

安全 SSO 应在服务器端实现，并提供更好的安全性：

```typescript
// 服务器端代码（Node.js/后端）
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 使用内置辅助创建安全 SSO
const userData = {
  id: 'user-123',
  email: 'john@example.com',
  username: 'john_doe',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg',
  isAdmin: false,
  isModerator: false
};

const sso = FastCommentsSSO.createSecure('your-api-key', userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoConfig = sso.prepareToSend();

// 在服务器上与 API 调用一起使用
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// 或将 ssoConfig 发送到客户端以供浏览器使用
```

### 在浏览器中使用 SSO（使用服务器生成的令牌）

```typescript
// 客户端代码（浏览器）
import { PublicApi } from 'fastcomments-sdk/browser';

// 从您的服务器端点获取 SSO 令牌
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### 带评论创建的 SSO

```typescript
// 服务器端：创建 SSO 和评论
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

const sso = FastCommentsSSO.createSecure('your-api-key', userData);
const ssoConfig = sso.prepareToSend();

const response = await publicApi.createCommentPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  broadcastId: 'unique-broadcast-id',
  commentData: {
    comment: 'This is my comment',
    date: Date.now(),
    commenterName: 'John Doe',
    url: 'https://example.com/page',
    urlId: 'page-url-id'
  },
  sso: JSON.stringify(ssoConfig)
});
```