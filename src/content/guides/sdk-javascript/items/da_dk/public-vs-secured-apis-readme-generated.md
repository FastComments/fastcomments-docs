SDK'en indeholder disse API-klasser:

- **`DefaultApi`** - Sikrede endpoints der kræver din API-nøgle for autentificering. Brug disse til server-side-operationer.
- **`PublicApi`** - Offentlige endpoints som kan tilgås uden en API-nøgle. Disse kan kaldes direkte fra browsere/mobilenheder/etc.
- **`ModerationApi`** - Moderator-dashboard endpoints (kommentarmodération, bans, badges, trust factor, søgning). Autentificeres via moderatorens session; send `sso` query-parametret for SSO-autentificerede moderatorer.
- **`HiddenApi`** - Interne/admin endpoints til avancerede brugsscenarier.

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Hent kommentarer for en side (ingen API-nøgle kræves)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Hold dette hemmeligt!
});
const defaultApi = new DefaultApi(config);

// Hent kommentarer med fuld administratoradgang
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath osv. */ });

// Moderator-autentificerede opkald (session-cookie, eller angiv `sso` for en SSO-moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```