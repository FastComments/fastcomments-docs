SDK pruža sledeće API klase:

- **`DefaultApi`** - Zaštićeni endpointi koji zahtevaju vaš API ključ za autentifikaciju. Koristite ih za operacije na serverskoj strani.
- **`PublicApi`** - Javni endpointi kojima se može pristupiti bez API ključa. Mogu se pozivati direktno iz pregledača/mobilnih uređaja itd.
- **`ModerationApi`** - Endpointi moderator dashboard-a (moderacija komentara, banovi, bedžovi, trust factor, pretraga). Autentifikuju se sesijom moderatora; prosledite `sso` query param za SSO-autentifikovane moderatore.
- **`HiddenApi`** - Interni/admin endpointi za napredne slučajeve upotrebe.

### Primer: Korišćenje Public API-ja (bezbedno za pregledač)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Get comments for a page (no API key required)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primer: Korišćenje Default API-ja (samo serverska strana)

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

### Primer: Korišćenje Moderation API-ja

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