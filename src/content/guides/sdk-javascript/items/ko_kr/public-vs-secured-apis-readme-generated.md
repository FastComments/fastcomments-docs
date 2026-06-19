The SDK provides these API classes:

- **`DefaultApi`** - 인증을 위해 API 키가 필요한 보안 엔드포인트입니다. 서버 측 작업에 사용하세요.
- **`PublicApi`** - API 키 없이 접근할 수 있는 공개 엔드포인트입니다. 브라우저/모바일 디바이스 등에서 직접 호출할 수 있습니다.
- **`ModerationApi`** - 모더레이터 대시보드 엔드포인트(댓글 관리, 차단, 배지, 신뢰도, 검색). 모더레이터의 세션으로 인증됩니다; SSO로 인증된 모더레이터의 경우 `sso` 쿼리 파라미터를 전달하세요.
- **`HiddenApi`** - 고급 사용 사례를 위한 내부/관리자용 엔드포인트입니다.

### 예제: Using Public API (브라우저 안전)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// 페이지의 댓글을 가져옵니다 (API 키 불필요)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 예제: Using Default API (서버 측 전용)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 비밀로 유지하세요!
});
const defaultApi = new DefaultApi(config);

// 전체 관리자 액세스 권한으로 댓글을 가져옵니다
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 예제: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath 등 */ });

// 모더레이터 인증 호출(세션 쿠키 또는 SSO 모더레이터의 경우 `sso` 전달).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```