Você verá que deve passar um `broadcastId` em algumas chamadas de API. Quando você receber eventos, receberá esse ID de volta, então saberá ignorar o evento se planejar aplicar mudanças de forma otimista no cliente (o que você provavelmente desejará fazer, pois oferece a melhor experiência). Passe um UUID aqui. O ID deve ser suficientemente único para não ocorrer duas vezes em uma sessão do navegador.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // ID único para esta operação
  }
});
```