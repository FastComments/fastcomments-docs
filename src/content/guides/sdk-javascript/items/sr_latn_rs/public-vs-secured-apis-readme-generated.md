SDK pruža sledeće API klase:

- **`DefaultApi`** - Zaštićeni endpointi koji zahtevaju vaš API ključ za autentifikaciju. Koristite ih za operacije na serverskoj strani.
- **`PublicApi`** - Javni endpointi kojima se može pristupiti bez API ključa. Mogu se pozivati direktno iz pregledača/mobilnih uređaja/itd.
- **`ModerationApi`** - Endpointi kontrolne table moderatora (moderacija komentara, zabrane, značke, faktor poverenja, pretraga). Autentifikacija putem sesije moderatora; prosledite query parametar `sso` za SSO autentifikovane moderatore.
- **`HiddenApi`** - Interni/admin endpointi za napredne slučajeve upotrebe.

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Dobavljanje komentara za stranicu (nije potreban API ključ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Čuvajte ovo u tajnosti!
});
const defaultApi = new DefaultApi(config);

// Dobavljanje komentara sa potpunim administratorskim pristupom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Pozivi autentifikovani kao moderator (kolačić sesije, ili prosledite `sso` za SSO moderatora).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```