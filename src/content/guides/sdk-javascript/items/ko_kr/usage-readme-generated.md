이 SDK는 브라우저 및 서버 환경에 대해 최적의 호환성과 보안을 보장하기 위해 별도의 진입점을 제공합니다:

### Browser Usage (Client-Side)

브라우저/프런트엔드 애플리케이션의 경우 Node.js 종속성을 제외한 브라우저 안전 익스포트를 사용하세요:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Server Usage (Node.js)

서버/백엔드 애플리케이션의 경우 SSO 및 인증 기능을 포함한 전체 SDK를 사용하세요:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Types Only Import

TypeScript 타입만 필요하고 런타임 코드는 필요하지 않다면(default import)을 사용하세요:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Using Individual API Classes

#### Browser Environment

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Server Environment  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```