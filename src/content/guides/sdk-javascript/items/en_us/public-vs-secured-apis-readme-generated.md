The SDK provides these API classes:

- **`DefaultApi`** - Secured endpoints that require your API key for authentication. Use these for server-side operations.
- **`PublicApi`** - Public endpoints that can be accessed without an API key. These can be called directly from browsers/mobile devices/etc.
- **`ModerationApi`** - Moderator dashboard endpoints (comment moderation, bans, badges, trust factor, search). Authenticated by the moderator's session; pass the `sso` query param for SSO-authenticated moderators.
- **`HiddenApi`** - Internal/admin endpoints for advanced use cases.

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Get comments for a page (no API key required)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Keep this secret!
});
const defaultApi = new DefaultApi(config);

// Get comments with full admin access
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Moderator-authenticated calls (session cookie, or pass `sso` for an SSO moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```