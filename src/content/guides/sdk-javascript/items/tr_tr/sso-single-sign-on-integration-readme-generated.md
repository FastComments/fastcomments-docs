FastComments, mevcut kullanıcı kimlik doğrulama sisteminizle entegre etmek için SSO'yu destekler. **SSO functionality is only available in the server export** since it requires Node.js crypto features.

### Simple SSO (Server-Side Only)

Basit SSO sunucu tarafında oluşturulmalı ve istemciye gönderilmelidir:

```typescript
// Sunucu tarafı kodu (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Yerleşik yardımcıyı kullanarak basit SSO oluştur  
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

// ssoToken'ı istemci tarafı kodunuza gönderin
// İstemci tarafı kodu daha sonra bu token'ı tarayıcı SDK'sı ile kullanabilir
```

### Secure SSO (Server-Side, Recommended)

Güvenli SSO sunucu tarafında uygulanmalı ve daha iyi güvenlik sağlar:

```typescript
// Sunucu tarafı kodu (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Yerleşik yardımcıyı kullanarak güvenli SSO oluştur
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

// Sunucuda API çağrılarıyla kullanın
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Veya tarayıcı kullanımı için ssoConfig'i istemciye gönderin
```

### Using SSO from Browser (with Server-Generated Token)

```typescript
// İstemci tarafı kodu (tarayıcı)
import { PublicApi } from 'fastcomments-sdk/browser';

// Sunucu uç noktanızdan SSO token'ını alın
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO with Comment Creation

```typescript
// Sunucu tarafı: SSO ve yorum oluştur
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