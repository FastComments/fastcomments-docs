FastComments 支援 SSO 以整合您現有的使用者認證系統。 **SSO 功能僅在伺服器端匯出時可用**，因為它需要 Node.js 的 crypto 功能。

### Simple SSO (Server-Side Only)

簡易 SSO 應在伺服器端產生並傳送給客戶端：

```typescript
// 伺服器端程式碼（Node.js/backend）
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 使用內建輔助函式建立簡易 SSO  
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

// 將 ssoToken 傳送到你的客戶端程式碼
// 客戶端程式碼之後可以使用此 token 與瀏覽器 SDK
```

### Secure SSO (Server-Side, Recommended)

安全 SSO 應在伺服器端實作，並提供較佳的安全性：

```typescript
// 伺服器端程式碼（Node.js/backend）
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 使用內建輔助函式建立安全 SSO
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

// 用於伺服器上的 API 呼叫
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// 或將 ssoConfig 傳送給客戶端供瀏覽器使用
```

### Using SSO from Browser (with Server-Generated Token)

```typescript
// 客戶端程式碼（瀏覽器）
import { PublicApi } from 'fastcomments-sdk/browser';

// 從你的伺服器端點取得 SSO token
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // 使用伺服器產生的 SSO token
});
```

### SSO with Comment Creation

```typescript
// 伺服器端：建立 SSO 與留言
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