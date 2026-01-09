Bu SDK, en iyi uyumluluk ve güvenliği sağlamak için tarayıcı ve sunucu ortamları için ayrı giriş noktaları sağlar:

### Tarayıcı Kullanımı (İstemci Tarafı)

Tarayıcı/ön yüz uygulamaları için Node.js bağımlılıklarını hariç tutan tarayıcıya güvenli dışa aktarmayı kullanın:

```typescript
// Tarayıcıya güvenli ithalat (Node.js bağımlılığı yok)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Tarayıcı SDK örneği oluştur
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // isteğe bağlı, varsayılan https://fastcomments.com
});

// Genel API'leri kullan (API anahtarına gerek yok - tarayıcılar için güvenli)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Sunucu Kullanımı (Node.js)

Sunucu/arka uç uygulamaları için SSO ve kimlik doğrulama özelliklerini içeren tam SDK'yı kullanın:

```typescript
// Sunucu tarafı ithalat (SSO içerir ve NodeJS ile çalışacak şekilde tasarlanmıştır)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Sunucu SDK örneği oluştur
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Sunucuda gizli tutun!
  basePath: 'https://fastcomments.com' // isteğe bağlı, varsayılan https://fastcomments.com
});

// API anahtarınızla güvenli API'leri kullan
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Sadece Tip İçe Aktarımı

Yalnızca TypeScript türlerine (çalışma zamanı kodu yok) ihtiyacınız varsa, varsayılan ithalatı kullanın:

```typescript
// Yalnızca tipler (çalışma zamanı bağımlılığı yok - her yerde güvenli)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Bireysel API Sınıflarını Kullanma

#### Tarayıcı Ortamı

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Sunucu Ortamı  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```