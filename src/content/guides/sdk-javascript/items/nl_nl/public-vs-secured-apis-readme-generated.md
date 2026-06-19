De SDK biedt deze API-klassen:

- **`DefaultApi`** - Beveiligde endpoints die uw API-sleutel vereisen voor authenticatie. Gebruik deze voor server-side operaties.
- **`PublicApi`** - Publieke endpoints die toegankelijk zijn zonder API-sleutel. Deze kunnen direct vanuit browsers/mobiele apparaten/etc. worden aangeroepen.
- **`ModerationApi`** - Moderator-dashboard endpoints (moderatie van opmerkingen, bans, badges, vertrouwensfactor, zoeken). Geauthenticeerd via de sessie van de moderator; geef de `sso` query-parameter door voor SSO-geauthenticeerde moderators.
- **`HiddenApi`** - Interne/admin endpoints voor geavanceerde gebruikssituaties.

### Voorbeeld: Public API gebruiken (browser-veilig)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Haal opmerkingen voor een pagina op (geen API-sleutel vereist)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Voorbeeld: Default API gebruiken (alleen server-side)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Houd deze geheim!
});
const defaultApi = new DefaultApi(config);

// Haal opmerkingen op met volledige admin-toegang
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Voorbeeld: Moderation API gebruiken

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, enz. */ });

// Aanroepen geauthenticeerd als moderator (sessie-cookie, of geef `sso` door voor een SSO-moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```