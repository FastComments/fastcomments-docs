Dieses SDK bietet getrennte Einstiegspunkte für Browser- und Server-Umgebungen, um optimale Kompatibilität und Sicherheit zu gewährleisten:

### Browser-Nutzung (Clientseitig)

Für Browser-/Frontend-Anwendungen verwenden Sie den browser-sicheren Export, der Node.js-Abhängigkeiten ausschließt:

```typescript
// Browser-sicherer Import (keine Node.js-Abhängigkeiten)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, Standardwert ist https://fastcomments.com
});

// Verwende öffentliche APIs (kein API-Schlüssel erforderlich - sicher für Browser)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Server-Nutzung (Node.js)

Für Server-/Backend-Anwendungen verwenden Sie das vollständige SDK mit SSO- und Authentifizierungsfunktionen:

```typescript
// Serverseitiger Import (inkl. SSO und für NodeJS ausgelegt)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Halten Sie diesen geheim auf dem Server!
  basePath: 'https://fastcomments.com' // optional, Standardwert ist https://fastcomments.com
});

// Verwenden Sie geschützte APIs mit Ihrem API-Schlüssel
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Nur Typen-Import

Wenn Sie nur TypeScript-Typen benötigen (kein Laufzeitcode), verwenden Sie den Standardimport:

```typescript
// Nur Typen (keine Laufzeitabhängigkeiten - überall sicher)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Verwendung einzelner API-Klassen

#### Browser-Umgebung

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Server-Umgebung  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```
---