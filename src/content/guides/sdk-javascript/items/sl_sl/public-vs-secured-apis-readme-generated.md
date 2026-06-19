The SDK zagotavlja naslednje API razrede:

- **`DefaultApi`** - Zaščitene končne točke, ki zahtevajo vaš API ključ za overjanje. Uporabite jih za operacije na strežniku.
- **`PublicApi`** - Javne končne točke, do katerih je mogoče dostopati brez API ključa. Te lahko kličete neposredno iz brskalnikov/mobilnih naprav/itd.
- **`ModerationApi`** - Končne točke nadzorne plošče moderatorja (moderacija komentarjev, prepovedi, značke, faktor zaupanja, iskanje). Avtenticirano z moderatorjevo sejo; posredujte poizvedni parameter `sso` za moderatorje, avtenticirane preko SSO.
- **`HiddenApi`** - Notranje/admin končne točke za napredne primere uporabe.

### Primer: Uporaba Public API (varno v brskalniku)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Pridobi komentarje za stran (API ključ ni potreben)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primer: Uporaba Default API (samo na strežniku)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Hranite to skrivno!
});
const defaultApi = new DefaultApi(config);

// Pridobi komentarje z polnim administratorskim dostopom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primer: Uporaba Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, itd. */ });

// Klici, avtenticirani z moderatorsko sejo (sejni piškotek) ali posredujte `sso` za moderatorja, avtenticiranega preko SSO.
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```