---
Das SDK stellt drei Haupt-API-Klassen bereit:

- **`DefaultApi`** - Gesicherte Endpunkte, die deinen API-Schlüssel zur Authentifizierung erfordern. Verwende diese für serverseitige Operationen.
- **`PublicApi`** - Öffentliche Endpunkte, die ohne API-Schlüssel zugänglich sind. Diese können direkt aus Browsern/Mobilgeräten/etc. aufgerufen werden.
- **`HiddenApi`** - Interne/Admin-Endpunkte für fortgeschrittene Anwendungsfälle.

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
  apiKey: 'your-api-key' // Dies geheim halten!
});
const defaultApi = new DefaultApi(config);

// Kommentare mit vollem Admin-Zugriff abrufen
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```
---