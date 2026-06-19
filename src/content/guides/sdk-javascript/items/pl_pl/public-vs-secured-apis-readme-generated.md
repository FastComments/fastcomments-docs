---
SDK udostępnia te klasy API:

- **`DefaultApi`** - Zabezpieczone punkty końcowe, które wymagają klucza API do uwierzytelniania. Używaj ich do operacji po stronie serwera.
- **`PublicApi`** - Publiczne punkty końcowe, do których można uzyskać dostęp bez klucza API. Można je wywoływać bezpośrednio z przeglądarek/urządzeń mobilnych itp.
- **`ModerationApi`** - Punkty końcowe panelu moderatora (moderacja komentarzy, bany, odznaki, wskaźnik zaufania, wyszukiwanie). Uwierzytelniane za pomocą sesji moderatora; przekaż parametr zapytania `sso` dla moderatorów uwierzytelnionych przez SSO.
- **`HiddenApi`** - Wewnętrzne/administacyjne punkty końcowe dla zaawansowanych zastosowań.

### Przykład: Korzystanie z Public API (bezpieczne dla przeglądarki)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Pobierz komentarze dla strony (klucz API nie jest wymagany)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Przykład: Korzystanie z Default API (tylko po stronie serwera)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Zachowaj to w tajemnicy!
});
const defaultApi = new DefaultApi(config);

// Pobierz komentarze z pełnym dostępem administratora
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Przykład: Korzystanie z Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath itp. */ });

// Wywołania uwierzytelnione jako moderator (ciasteczko sesyjne, lub przekaż `sso` dla moderatora uwierzytelnionego przez SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```
---