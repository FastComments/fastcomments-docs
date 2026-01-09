FastComments는 기존 사용자 인증 시스템과 통합하기 위해 SSO를 지원합니다. **SSO 기능은 Node.js의 crypto 기능을 필요로 하기 때문에 서버 내보내기에서만 사용할 수 있습니다.**

### 간단한 SSO (서버 전용)

간단한 SSO는 서버 측에서 생성되어 클라이언트로 전송되어야 합니다:

```typescript
// 서버 측 코드 (Node.js/백엔드)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create simple SSO using the built-in helper  
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

// ssoToken을 클라이언트 측 코드로 전송
// 클라이언트 측 코드는 이 토큰을 브라우저 SDK와 함께 사용할 수 있음
```

### 보안 SSO (서버 측, 권장)

보안 SSO는 서버 측에 구현되어야 하며 더 나은 보안을 제공합니다:

```typescript
// 서버 측 코드 (Node.js/백엔드)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create secure SSO using the built-in helper
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

// 서버에서 API 호출과 함께 사용
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// 또는 브라우저 사용을 위해 ssoConfig를 클라이언트로 전송
```

### 브라우저에서 SSO 사용 (서버 생성 토큰 포함)

```typescript
// 클라이언트 측 코드 (브라우저)
import { PublicApi } from 'fastcomments-sdk/browser';

// 서버 엔드포인트에서 SSO 토큰 가져오기
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // 서버에서 생성된 SSO 토큰 사용
});
```

### 댓글 생성과 함께하는 SSO

```typescript
// 서버 측: SSO 및 댓글 생성
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