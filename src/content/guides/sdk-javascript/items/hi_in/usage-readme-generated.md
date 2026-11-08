यह SDK ब्राउज़र और सर्वर वातावरणों के लिए अलग-अलग एंट्री पॉइंट प्रदान करता है ताकि इष्टतम संगतता और सुरक्षा सुनिश्चित हो:

### ब्राउज़र उपयोग (क्लाइंट-साइड)

ब्राउज़र/फ्रंटएंड एप्लिकेशन के लिए, ब्राउज़र-सेफ एक्सपोर्ट का उपयोग करें जो Node.js डिपेंडेंसीज़ को बाहर रखता है:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### सर्वर उपयोग (Node.js)

सर्वर/बैकएंड एप्लिकेशन के लिए, SSO और प्रमाणीकरण सुविधाओं वाले पूर्ण SDK का उपयोग करें:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### केवल टाइप्स इंपोर्ट

यदि आपको केवल TypeScript टाइप्स की आवश्यकता है (कोई रनटाइम कोड नहीं), तो डिफ़ॉल्ट इंपोर्ट का उपयोग करें:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### अलग-अलग API क्लासेस का उपयोग

#### ब्राउज़र वातावरण

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### सर्वर वातावरण  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```