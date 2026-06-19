Das SDK stellt diese API-Klassen bereit:

- **`DefaultApi`** - Gesicherte Endpunkte, die Ihren API-Schlüssel zur Authentifizierung benötigen. Verwenden Sie diese für serverseitige Vorgänge.
- **`PublicApi`** - Öffentliche Endpunkte, die ohne API-Schlüssel zugänglich sind. Diese können direkt von Browsern/Mobilgeräten/etc. aufgerufen werden.
- **`ModerationApi`** - Moderator-Dashboard-Endpunkte (Kommentar-Moderation, Sperren, Abzeichen, Vertrauensfaktor, Suche). Authentifiziert durch die Sitzung des Moderators; übergeben Sie den `sso`-Abfrageparameter für SSO-authentifizierte Moderatoren.
- **`HiddenApi`** - Interne/Admin-Endpunkte für erweiterte Anwendungsfälle.

### Beispiel: Verwendung der Public API (browser-sicher)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Kommentare für eine Seite abrufen (kein API-Schlüssel erforderlich)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Beispiel: Verwendung der Default API (nur serverseitig)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Geheim halten!
});
const defaultApi = new DefaultApi(config);

// Kommentare mit vollem Administratorzugriff abrufen
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Beispiel: Verwendung der Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath usw. */ });

// Durch Moderator authentifizierte Aufrufe (Session-Cookie oder `sso` für einen SSO-Moderator übergeben).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```