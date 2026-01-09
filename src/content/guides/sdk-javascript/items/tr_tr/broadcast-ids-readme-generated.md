Bazı API çağrılarında bir `broadcastId` geçirmeniz gerektiğini göreceksiniz. Etkinlikleri aldığınızda bu ID size geri gönderilir; bu sayede istemcide değişiklikleri iyimser şekilde uygulamayı planlıyorsanız o etkinliği görmezden geleceğinizi bilirsiniz (muhtemelen en iyi deneyimi sunduğu için bunu yapmak isteyeceksiniz). Burada bir UUID iletin. ID, bir tarayıcı oturumu içinde iki kez oluşmayacak kadar benzersiz olmalıdır.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Bu işlem için benzersiz ID
  }
});
```