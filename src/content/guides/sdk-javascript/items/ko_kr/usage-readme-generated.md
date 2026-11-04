이 SDK는 최적의 호환성과 보안을 보장하기 위해 브라우저 환경과 서버 환경에 대해 별도의 진입점을 제공합니다:

### 브라우저 사용 (클라이언트 사이드)

브라우저/프론트엔드 애플리케이션의 경우, Node.js 의존성을 제외한 브라우저 안전 export를 사용하세요:

```typescript
// 브라우저 안전 import (Node.js 의존성 없음)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// 브라우저 SDK 인스턴스 생성
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // 선택 사항, 기본값은 https://fastcomments.com
});

// 공개 API 사용 (API 키 불필요 - 브라우저에서 안전)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 서버 사용 (Node.js)

서버/백엔드 애플리케이션의 경우, SSO 및 인증 기능이 포함된 전체 SDK를 사용하세요:

```typescript
// 서버 사이드 import (SSO 포함, NodeJS와 함께 작동하도록 설계됨)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// 서버 SDK 인스턴스 생성
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // 서버에서 비밀로 유지하세요!
  basePath: 'https://fastcomments.com' // 선택 사항, 기본값은 https://fastcomments.com
});

// API 키로 보안이 적용된 API 사용
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 타입 전용 Import

TypeScript 타입만 필요한 경우(런타임 코드 없음), 기본 import를 사용하세요:

```typescript
// 타입 전용 (런타임 의존성 없음 - 어디서든 안전)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### 개별 API 클래스 사용

#### 브라우저 환경

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### 서버 환경  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```