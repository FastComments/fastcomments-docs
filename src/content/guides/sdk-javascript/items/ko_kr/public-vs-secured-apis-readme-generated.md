The SDK는 세 가지 주요 API 클래스를 제공합니다:

- **`DefaultApi`** - 인증을 위해 API 키가 필요한 보안 엔드포인트입니다. 서버 측 작업에 사용하세요.
- **`PublicApi`** - API 키 없이 접근 가능한 공개 엔드포인트입니다. 브라우저/모바일 기기 등에서 직접 호출할 수 있습니다.
- **`HiddenApi`** - 고급 사용 사례를 위한 내부/관리자용 엔드포인트입니다.

### 예시: Public API 사용 (브라우저에서 안전하게 사용)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// 페이지의 댓글을 가져옵니다 (API 키 필요 없음)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 예시: Default API 사용 (서버 측 전용)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // 비밀로 유지하세요!
});
const defaultApi = new DefaultApi(config);

// 전체 관리자 권한으로 댓글을 가져옵니다
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```