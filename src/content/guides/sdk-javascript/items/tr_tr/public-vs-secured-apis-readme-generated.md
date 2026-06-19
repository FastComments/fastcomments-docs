---
SDK şu API sınıflarını sağlar:

- **`DefaultApi`** - Kimlik doğrulama için API anahtarınızı gerektiren güvenli uç noktalar. Sunucu tarafı işlemler için bunları kullanın.
- **`PublicApi`** - API anahtarı olmadan erişilebilen genel uç noktalar. Bunlar doğrudan tarayıcılardan/mobil cihazlardan/vb. çağrılabilir.
- **`ModerationApi`** - Moderatör paneli uç noktaları (yorum moderasyonu, yasaklar, rozetler, güven faktörü, arama). Moderatörün oturumu ile yetkilendirilir; SSO ile yetkilendirilmiş moderatörler için `sso` sorgu parametresini geçin.
- **`HiddenApi`** - Gelişmiş kullanım durumları için dahili/admin uç noktaları.

### Örnek: Public API Kullanımı (tarayıcıda güvenli)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Bir sayfa için yorumları al (API anahtarı gerekmez)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Örnek: Default API Kullanımı (sadece sunucu tarafı)

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

### Örnek: Moderation API Kullanımı

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, vb. */ });

// Moderatör kimliğiyle yetkilendirilmiş çağrılar (oturum çerezi, veya SSO moderatörü için `sso`'yu geçin).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```
---