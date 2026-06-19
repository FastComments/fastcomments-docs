SDK pruža ove API klase:

- **`DefaultApi`** - Zaštićeni endpointi koji zahtijevaju vaš API ključ za autentikaciju. Koristite ih za operacije na strani poslužitelja.
- **`PublicApi`** - Javni endpointi kojima se može pristupiti bez API ključa. Mogu se pozivati izravno iz preglednika/mobilnih uređaja/itd.
- **`ModerationApi`** - Endpointi nadzorne ploče moderatora (moderacija komentara, zabrane, značke, faktor povjerenja, pretraživanje). Autentificiraju se putem moderatorove sesije; proslijedite parametar upita `sso` za moderatore autentificirane putem SSO.
- **`HiddenApi`** - Interni/admin endpointi za napredne slučajeve upotrebe.

### Primjer: Korištenje Public API-ja (sigurno za preglednik)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Dohvati komentare za stranicu (nije potreban API ključ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primjer: Korištenje Default API-ja (samo na strani poslužitelja)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Držite ovo u tajnosti!
});
const defaultApi = new DefaultApi(config);

// Dohvati komentare s punim administratorskim pristupom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primjer: Korištenje Moderation API-ja

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath itd. */ });

// Pozivi autentificirani moderatorom (sesijski kolačić, ili proslijedite `sso` za SSO moderatora).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```