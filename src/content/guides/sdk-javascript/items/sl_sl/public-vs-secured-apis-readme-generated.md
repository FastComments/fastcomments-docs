SDK zagotavlja tri glavne razrede API:

- **`DefaultApi`** - Zavarovani končni točki, ki zahtevata vaš API ključ za avtentikacijo. Uporabite jih za operacije na strežniški strani.
- **`PublicApi`** - Javne končne točke, do katerih je mogoče dostopati brez API ključa. Klice lahko pošljete neposredno iz brskalnikov/mobilnih naprav/itd.
- **`HiddenApi`** - Notranje/skrbniške končne točke za napredne primere uporabe.

### Primer: Uporaba javnega API-ja (varno za brskalnik)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Pridobi komentarje za stran (API ključ ni potreben)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primer: Uporaba privzetega API-ja (samo na strežniku)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // To naj ostane skrivnost!
});
const defaultApi = new DefaultApi(config);

// Pridobi komentarje s popolnim skrbniškim dostopom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```