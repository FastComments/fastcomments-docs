SDK üç ana API sınıfı sağlar:

- **`DefaultApi`** - Kimlik doğrulama için API anahtarınızın gerektiği güvenli uç noktalar. Sunucu tarafı işlemler için bunları kullanın.
- **`PublicApi`** - API anahtarı olmadan erişilebilen genel uç noktalar. Bunlar tarayıcılardan/taşınabilir cihazlardan vb. doğrudan çağrılabilir.
- **`HiddenApi`** - İleri düzey kullanım durumları için dahili/yönetici uç noktaları.

### Örnek: Public API Kullanımı (tarayıcı için güvenli)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Bir sayfanın yorumlarını al (API anahtarı gerekli değil)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Örnek: Default API Kullanımı (yalnızca sunucu tarafı)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Bunu gizli tutun!
});
const defaultApi = new DefaultApi(config);

// Tam yönetici erişimi ile yorumları al
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```