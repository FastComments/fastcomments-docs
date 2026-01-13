FastComments は既存のユーザー認証システムと統合するための SSO をサポートしています。**SSO 機能は Node.js の crypto 機能を必要とするため、サーバーエクスポートでのみ利用可能です。**

### Simple SSO（サーバーサイドのみ）

Simple SSO はサーバーサイドで生成してクライアントに送信する必要があります：

```typescript
// サーバー側のコード (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 組み込みのヘルパーを使って simple SSO を作成  
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

// ssoToken をクライアント側コードに送信する
// クライアント側のコードはこのトークンをブラウザ用 SDK で使用できます
```

### Secure SSO（サーバーサイド、推奨）

Secure SSO はサーバーサイドで実装するべきで、より高いセキュリティを提供します：

```typescript
// サーバー側のコード (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// 組み込みのヘルパーを使って secure SSO を作成
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

// サーバー上の API 呼び出しで使用
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// または ssoConfig をクライアントに送ってブラウザで使用
```

### ブラウザからの SSO の使用（サーバー生成トークンあり）

```typescript
// クライアント側のコード（ブラウザ）
import { PublicApi } from 'fastcomments-sdk/browser';

// サーバーのエンドポイントから SSO トークンを取得
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // サーバー生成の SSO トークンを使用
});
```

### コメント作成時の SSO

```typescript
// サーバー側: SSO とコメントを作成
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