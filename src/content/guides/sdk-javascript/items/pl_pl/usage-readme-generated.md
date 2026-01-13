To SDK zapewnia oddzielne punkty wejścia dla środowisk przeglądarki i serwera, aby zapewnić optymalną zgodność i bezpieczeństwo:

### Użycie w przeglądarce (po stronie klienta)

Dla aplikacji w przeglądarce/front-end użyj eksportu bezpiecznego dla przeglądarki, który wyklucza zależności Node.js:

```typescript
// Import bezpieczny dla przeglądarki (bez zależności Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Utwórz instancję SDK dla przeglądarki
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opcjonalnie, domyślnie https://fastcomments.com
});

// Używaj publicznych API (brak potrzeby klucza API - bezpieczne w przeglądarkach)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Użycie po stronie serwera (Node.js)

Dla aplikacji serwerowych/back-end użyj pełnego SDK z funkcjami SSO i uwierzytelniania:

```typescript
// Import po stronie serwera (zawiera SSO i zaprojektowany do pracy z NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Utwórz instancję SDK dla serwera
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Przechowuj to w tajemnicy na serwerze!
  basePath: 'https://fastcomments.com' // opcjonalnie, domyślnie https://fastcomments.com
});

// Używaj zabezpieczonych API z Twoim kluczem API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Tylko import typów

Jeśli potrzebujesz tylko typów TypeScript (bez kodu uruchomieniowego), użyj domyślnego importu:

```typescript
// Tylko typy (bez zależności w czasie wykonywania - bezpieczne wszędzie)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Korzystanie z poszczególnych klas API

#### Środowisko przeglądarki

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Środowisko serwera  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```