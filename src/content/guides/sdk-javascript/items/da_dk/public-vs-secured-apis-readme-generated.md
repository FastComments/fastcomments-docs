SDK'en giver disse API-klasser:

- **`DefaultApi`** - Sikrede endpoints, der kræver din API-nøgle til autentificering. Brug disse til server-side-operationer.
- **`PublicApi`** - Offentlige endpoints, der kan tilgås uden en API-nøgle. Disse kan kaldes direkte fra browsere/mobilenheder/osv.
- **`ModerationApi`** - Moderator-dashboard-endpoints (kommentar-moderering, udeladelser, badges, trust factor, søgning). Autentificeret via moderatorens session; send `sso`-query-parameteren for SSO-autentificerede moderatorer.
- **`HiddenApi`** - Interne/admin-endpoints til avancerede brugsscenarier.

### Eksempel: Brug af Public API (browser-sikker)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Hent kommentarer for en side (ingen API-nøgle påkrævet)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Eksempel: Brug af Default API (kun server-side)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Hold dette hemmeligt!
});
const defaultApi = new DefaultApi(config);

// Hent kommentarer med fuld admin-adgang
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Eksempel: Brug af Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, osv. */ });

// Moderator-autentificerede kald (session-cookie, eller send `sso` for en SSO-moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```